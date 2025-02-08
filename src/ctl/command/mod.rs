use crate::connection::{HyprlandConnection, HyprlandError};
use std::io;

mod commands;
pub use super::arguments::*;
pub use commands::*;

pub enum CommandError {
    HyprlandError(HyprlandError),
    IOError(io::Error),
}

impl HyprlandConnection {
    pub async fn dispatch<T: DispatchCommand + ?Sized>(
        &self,
        command: &T,
    ) -> Result<(), CommandError> {
        match self
            .send_raw_message(format!("dispatch {}", command.get_command()).as_str())
            .await
        {
            Ok(s) if s.starts_with("ok") => Ok(()),
            Err(e) => Err(CommandError::IOError(e)),
            Ok(error) => Err(CommandError::HyprlandError(HyprlandError::new(error))),
        }
    }
}
