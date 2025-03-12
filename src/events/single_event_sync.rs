use std::{
    io::{self, BufRead, BufReader},
    os::unix::net::UnixStream,
};

use crate::HyprlandConnection;

use super::{common::parse_event, EventFilter, HyprlandEvent};

/// Represents a connection to the event socket.
/// Useful for receiving one event at the time or having a mutable filter.
pub struct DetachedEventConnection {
    socket: BufReader<UnixStream>,
    filter: EventFilter,
}

impl Iterator for DetachedEventConnection {
    type Item = Result<HyprlandEvent, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::with_capacity(1024);
        match self.socket.read_line(&mut buf) {
            Ok(len) => {
                let event = parse_event(&buf[..len], &self.filter);
                if let Ok(event) = event {
                    Some(Ok(event))
                } else {
                    None
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}

impl DetachedEventConnection {
    /// Connects to the event socket of the HyprlandConnection and applies the filter when
    /// listening for events.
    pub fn from_connection(
        conn: HyprlandConnection,
        filter: EventFilter,
    ) -> Result<Self, io::Error> {
        let path = conn.get_event_socket_path()?;

        Ok(DetachedEventConnection {
            socket: BufReader::new(UnixStream::connect(path)?),
            filter,
        })
    }
}
