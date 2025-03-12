use std::io;
use std::str;

use super::common::*;
use crate::connection::HyprlandConnection;

use log::warn;
use tokio::io::Interest;
use tokio::net::UnixStream;
use tokio::sync::broadcast;
use tokio::time;

pub use super::HyprlandEvent;

impl HyprlandConnection {
    /// Spawns a task that listens to Hyprland events and sends them through an async channel. If a
    /// connection already exists, it gets restarted with the new filter provided.
    ///
    /// [`stop_listening`]: #method.stop_listening
    pub async fn listen_to_events(
        &mut self,
        filter: EventFilter,
    ) -> Result<broadcast::Receiver<HyprlandEvent>, io::Error> {
        if self.event_handle.is_some() {
            self.stop_listening();
        }

        let path = self.get_event_socket_path()?;
        let socket = UnixStream::connect(path).await?;

        let (tx, rx) = broadcast::channel(64);

        let abort_handle = tokio::spawn(async move {
            'main_loop: loop {
                socket.ready(Interest::READABLE).await.unwrap();

                let mut buf = [0; 1024];
                match socket.try_read(&mut buf) {
                    Ok(len) => {
                        let str_buf = str::from_utf8(&buf[..len])
                            .unwrap()
                            .strip_suffix('\n')
                            .unwrap();
                        for line in str_buf.split('\n') {
                            if let Ok(event) = parse_event(line, &filter) {
                                if tx.send(event).is_err() {
                                    break 'main_loop;
                                } else if tx.len() >= 64 {
                                    warn!("Event channel is full");
                                }
                            }
                        }
                    }
                    Err(_) => {
                        time::sleep(time::Duration::from_millis(100)).await;
                    }
                }
            }
        })
        .abort_handle();

        self.event_handle = Some(abort_handle);

        Ok(rx)
    }

    /// Returns whether this connection is currently listening to events or not
    pub fn is_listening_to_events(&self) -> bool {
        if let Some(abort_handle) = self.event_handle.as_ref() {
            !abort_handle.is_finished()
        } else {
            false
        }
    }

    /// Aborts the task that is currently sending events through the broadcast channel. The
    /// broadcast channel also closes. If there is no connection to the event socket, this function
    /// does nothing.
    pub fn stop_listening(&mut self) {
        if let Some(abort_handle) = self.event_handle.as_ref() {
            abort_handle.abort();
            self.event_handle = None;
        }
    }
}
