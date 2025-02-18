use crate::connection::{HyprlandConnection, HyprlandError};
use std::io;

mod commands;
pub use super::arguments::*;
pub use commands::*;

pub enum CommandError {
    HyprlandError(HyprlandError),
    IOError(io::Error),
    LibraryError(String),
}

impl HyprlandConnection {
    /// Send a dispatch command to Hyprland
    ///
    /// # Example
    ///
    /// ```
    /// use hyprrust::connection::HyprlandConnection;
    /// use hyprrust::ctl::command::*;
    /// let conn = HyprlandConnection::new();
    /// conn.dispatch(MoveWindow::with_direction(DirectionArgument::Left))
    /// ```
    pub async fn send_command<T: Command + ?Sized>(&self, command: &T) -> Result<(), CommandError> {
        let prefix = match command.get_type() {
            CommandType::DispatchCommand => "dispatch ",
            CommandType::DirectCommand => "",
        };

        match self
            .send_raw_message(format!("{}{}", prefix, command.get_command()).as_str())
            .await
        {
            Ok(s) if s.starts_with("ok") => Ok(()),
            Err(e) => Err(CommandError::IOError(e)),
            Ok(error) => Err(CommandError::HyprlandError(HyprlandError::new(error))),
        }
    }
}
