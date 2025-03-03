use std::io::{self, Read, Write};

use std::os::unix::net::UnixStream as SyncUnixStream;

#[cfg(feature = "async")]
use tokio::{io::Interest, net::UnixStream};

use crate::connection::HyprlandConnection;

pub mod arguments;
pub mod commands;
pub mod data;

impl HyprlandConnection {
    /// Works the same as [`send_raw_message`].
    ///
    /// [`send_raw_message`]: #method.send_raw_message
    #[cfg(feature = "sync")]
    pub fn send_raw_message_sync(&self, msg: &str) -> Result<String, io::Error> {
        let path = self.get_ctl_socket_path()?;

        let mut socket = SyncUnixStream::connect(path)?;

        match socket.write(msg.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        let mut buf = String::new();
        match socket.read_to_string(&mut buf) {
            Ok(_) => Ok(buf),
            Err(e) => Err(e),
        }
    }

    /// Sends a string to the hyprland socket not checking anything besides io errors.
    #[cfg(feature = "async")]
    pub async fn send_raw_message(&self, msg: &str) -> Result<String, io::Error> {
        let mut path = self.get_ctl_socket_path()?;
        path.push(".socket.sock");

        let socket = UnixStream::connect(path).await?;

        loop {
            socket.ready(Interest::WRITABLE).await?;

            match socket.try_write(msg.as_bytes()) {
                Ok(_) => {
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        'ready_loop: loop {
            socket.ready(Interest::READABLE).await?;

            let mut block_buf = Vec::with_capacity(2048);
            let mut buf = Vec::new();
            loop {
                match socket.try_read_buf(&mut block_buf) {
                    Ok(0) => {
                        return Ok(String::from_utf8_lossy(&buf).into_owned());
                    }
                    Ok(_) => {
                        buf.append(&mut block_buf);
                        continue;
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue 'ready_loop;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        }
    }
}
