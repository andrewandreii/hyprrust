use std::io;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixStream;

use crate::HyprlandConnection;

use super::common::*;
use super::HyprlandEvent;

/// Represents a connection to the event socket.
/// Useful for receiving one event at the time or having a mutable filter.
pub struct DetachedEventConnection {
    socket: BufReader<UnixStream>,
    filter: EventFilter,
}

impl DetachedEventConnection {
    /// Connects to the event socket of the HyprlandConnection and applies the filter when
    /// listening for events.
    pub async fn from_connection(
        conn: HyprlandConnection,
        filter: Option<EventFilter>,
    ) -> Result<Self, io::Error> {
        let path = conn.get_event_socket_path()?;

        let filter = filter.unwrap_or_else(|| EventFilter::new_include_all());

        Ok(DetachedEventConnection {
            socket: BufReader::new(UnixStream::connect(path).await?),
            filter,
        })
    }

    /// Returns the next event that gets sent over the socket.
    pub async fn next_event(&mut self) -> Result<HyprlandEvent, io::Error> {
        loop {
            let mut buf = String::with_capacity(1024);
            match self.socket.read_line(&mut buf).await {
                Ok(len) => {
                    if let Ok(event) = parse_event(&buf[..len], &self.filter) {
                        return Ok(event);
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}
