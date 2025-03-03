use crate::connection::HyprlandConnection;
use crate::errors::{CommandError, HyprlandError};
use std::ops::Deref;

mod commands;
use super::arguments::*;
pub use commands::*;

/// A vector of commands
pub type Recipe = Vec<Box<dyn Command>>;
/// Creates a Recipe (`Vec<Box<dyn Command>>`). Just a shorthand so that you don't have to call
/// `Box::new`
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
    /// Send a dispatch command to Hyprland.
    ///
    /// ```
    #[doc = include_str!("../../../examples/simple_command.rs")]
    /// ```
    #[cfg(feature = "async")]
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

    /// The blocking counterpart of [`send_command`].
    ///
    /// [`send_command`]: #method.send_command
    #[cfg(feature = "sync")]
    pub fn send_command_sync<T: Command + ?Sized>(&self, command: &T) -> Result<(), CommandError> {
        match self.send_raw_message_sync(
            format!("{}{}", get_command_prefix(command), command.get_command()).as_str(),
        ) {
            Ok(s) if s.starts_with("ok") => Ok(()),
            Err(e) => Err(CommandError::IOError(e)),
            Ok(error) => Err(CommandError::HyprlandError(HyprlandError::new(error))),
        }
    }

    /// Sends a list of commands to the socket at once. This is faster than sending each command
    /// separately.
    #[cfg(feature = "async")]
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

    /// The blocking counterpart of [`send_recipe`].
    ///
    /// [`send_recipe`]: #method.send_recipe
    #[cfg(feature = "sync")]
    pub fn send_recipe_sync(&self, recipe: &Recipe) -> Result<(), Vec<CommandError>> {
        let resp = self.send_raw_message_sync(get_batch_from_recipe(recipe).as_str());

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

fn get_batch_from_recipe(recipe: &Recipe) -> String {
    let mut full_command = String::from("/[[BATCH]]");
    for command in recipe {
        full_command.push_str(get_command_prefix(command.deref()));
        full_command.push_str(command.get_command().as_str());
        full_command.push(';');
    }

    full_command
}
