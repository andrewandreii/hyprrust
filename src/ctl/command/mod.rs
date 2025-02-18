use crate::{
    connection::{HyprlandConnection, HyprlandError},
    LibraryError,
};
use core::fmt;
use std::{error::Error, io, ops::Deref};

mod commands;
pub use super::arguments::*;
pub use commands::*;

#[derive(Debug)]
pub enum CommandError {
    HyprlandError(HyprlandError),
    IOError(io::Error),
    LibraryError(LibraryError),
}
impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl Error for CommandError {}

pub type Recipe = Vec<Box<dyn Command>>;
#[macro_export]
macro_rules! recipe {
    [] => {
        vec![];
    };
    [$elem:expr; $n:expr] => {
        vec![$elem; $n];
    };
    [$($cmd:expr),* $(,)?] => {
        vec![$(Box::new($cmd)),*]
    };
}
pub use recipe;

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
        match self
            .send_raw_message(
                format!("{}{}", get_command_prefix(command), command.get_command()).as_str(),
            )
            .await
        {
            Ok(s) if s.starts_with("ok") => Ok(()),
            Err(e) => Err(CommandError::IOError(e)),
            Ok(error) => Err(CommandError::HyprlandError(HyprlandError::new(error))),
        }
    }

    pub async fn send_recipe(&self, recipe: &Recipe) -> Result<(), Vec<CommandError>> {
        let resp = self
            .send_raw_message(get_batch_from_recipe(recipe).as_str())
            .await;

        match resp {
            Ok(resp) => {
                let errors = resp
                    .split("\n\n\n")
                    .filter(|resp| resp != &"ok")
                    .map(|resp| CommandError::HyprlandError(HyprlandError::new(resp.to_string())))
                    .collect::<Vec<CommandError>>();
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
            Err(e) => Err(vec![CommandError::IOError(e)]),
        }
    }
}

fn get_command_prefix<T: Command + ?Sized>(cmd: &T) -> &'static str {
    match cmd.get_type() {
        CommandType::DispatchCommand => "dispatch ",
        CommandType::DirectCommand => "",
    }
}

pub fn get_batch_from_recipe(recipe: &Recipe) -> String {
    let mut full_command = String::from("/[[BATCH]]");
    for command in recipe {
        full_command.push_str(get_command_prefix(command.deref()));
        full_command.push_str(command.get_command().as_str());
        full_command.push(';');
    }

    full_command
}
