use core::panic;
use std::io;
use std::str;

use super::common::*;
use crate::connection::EventConnection;
use crate::connection::HyprlandConnection;

use tokio::io::Interest;
use tokio::net::UnixStream;
use tokio::sync::broadcast;
use tokio::time;

pub use super::HyprlandEvent;

impl HyprlandConnection {
    /// Spawns a task that listens to Hyprland events and sends them through an async channel. If a
    /// connection already exists, it gets restarted with the new filter provided.
    ///
    /// Once created, the listening task won't die even if all receivers have been dropped. To
    /// kill it call [`stop_listening`].
    ///
    /// [`stop_listening`]: #method.stop_listening
    pub async fn listen_to_events(
        &mut self,
        filter: Option<EventFilter>,
    ) -> Result<broadcast::Receiver<HyprlandEvent>, io::Error> {
        if self.event_connection.is_some() {
            self.stop_listening();
        }

        let path = self.get_event_socket_path()?;
        let socket = UnixStream::connect(path).await?;

        let (tx, rx) = broadcast::channel(16);

        let filter = filter.unwrap_or_else(EventFilter::new_include_all);

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

        let rx_clone = rx.resubscribe();
        self.event_connection = Some(EventConnection {
            abort_handle,
            receiver: rx,
        });

        Ok(rx_clone)
    }

    /// Resubscribes to the event receiver. Must be called after [`listen_to_events`].
    ///
    /// [`listen_to_events`]: #method.listen_to_events
    pub fn resubscribe_to_events(&self) -> broadcast::Receiver<HyprlandEvent> {
        if let Some(ev_conn) = &self.event_connection {
            ev_conn.receiver.resubscribe()
        } else {
            panic!("Listener task hasn't been started");
        }
    }

    /// Returns whether this connection is currently listening to events or not
    pub fn is_listening_to_events(&self) -> bool {
        if let Some(event_connection) = self.event_connection.as_ref() {
            !event_connection.abort_handle.is_finished()
        } else {
            false
        }
    }

    /// Aborts the task that is currently sending events through the broadcast channel. The
    /// broadcast channel also closes. If there is no connection to the event socket, this function
    /// does nothing.
    pub fn stop_listening(&mut self) {
        if let Some(event_connection) = self.event_connection.as_ref() {
            event_connection.abort_handle.abort();
            self.event_connection = None;
        }
    }
}
