use std::io;
use std::task::Poll;

use futures::future::BoxFuture;
use futures::ready;
use futures::FutureExt;
use futures::Stream;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixStream;

use crate::HyprlandConnection;

use super::common::*;
use super::HyprlandEvent;

async fn receive(
    mut reader: BufReader<UnixStream>,
    filter: EventFilter,
) -> (
    BufReader<UnixStream>,
    Result<HyprlandEvent, io::Error>,
    EventFilter,
) {
    let mut buf = String::with_capacity(1024);
    loop {
        match reader.read_line(&mut buf).await {
            Ok(len) => {
                let event = parse_event(&buf[..len], &filter);
                if let Ok(event) = event {
                    return (reader, Ok(event), filter);
                } else {
                    continue;
                }
            }
            Err(e) => {
                return (reader, Err(e), filter);
            }
        }
    }
}

/// Represents a connection to the event socket.
/// Useful for receiving one event at the time or having a mutable filter.
pub struct DetachedEventConnection<'a>(
    BoxFuture<
        'a,
        (
            BufReader<UnixStream>,
            Result<HyprlandEvent, io::Error>,
            EventFilter,
        ),
    >,
);

impl Stream for DetachedEventConnection<'_> {
    type Item = Result<HyprlandEvent, io::Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let (reader, result, filter) = ready!(self.0.poll_unpin(cx));

        self.0 = receive(reader, filter).boxed();

        Poll::Ready(Some(result))
    }
}

impl DetachedEventConnection<'_> {
    /// Connects to the event socket of the HyprlandConnection and applies the filter when
    /// listening for events.
    pub async fn from_connection(
        conn: HyprlandConnection,
        filter: EventFilter,
    ) -> Result<Self, io::Error> {
        let path = conn.get_event_socket_path()?;

        Ok(DetachedEventConnection(
            receive(BufReader::new(UnixStream::connect(path).await?), filter).boxed(),
        ))
    }
}
