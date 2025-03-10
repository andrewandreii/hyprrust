use std::io;

use crate::connection::HyprlandConnection;
use crate::errors::{CommandError, HyprlandError};

mod all_commands;
use super::arguments::*;
pub use all_commands::*;

impl HyprlandConnection {
    /// Send a dispatch command to Hyprland.
    ///
    /// ```
    #[doc = include_str!("../../../examples/simple_command.rs")]
    /// ```
    #[cfg(feature = "async")]
    pub async fn send_command(&self, command: &Command) -> Result<(), CommandError> {
        check_hyprland_response(self.send_raw_message(command.get_command()).await)
    }

    /// The blocking counterpart of [`Self::send_command`].
    #[cfg(feature = "sync")]
    pub fn send_command_sync(&self, command: &Command) -> Result<(), CommandError> {
        check_hyprland_response(self.send_raw_message_sync(command.get_command()))
    }

    /// Sends a list of commands to the socket at once. This is faster than sending each command
    /// separately.
    #[cfg(feature = "async")]
    pub async fn send_recipe(&self, recipe: &[Command]) -> Result<(), Vec<CommandError>> {
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

    /// The blocking counterpart of [`Self::send_recipe`].
    #[cfg(feature = "sync")]
    pub fn send_recipe_sync(&self, recipe: &[Command]) -> Result<(), Vec<CommandError>> {
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

    /// Sets the config variable named `variable` to `value`. Can be used instead of
    /// [`Self::send_command`] with [`SetConfigValue`].
    pub async fn set_config_variable(
        &self,
        variable: &str,
        value: &str,
    ) -> Result<(), CommandError> {
        check_hyprland_response(
            self.send_raw_message(format!("keyword {} {}", variable, value).as_str())
                .await,
        )
    }

    /// Blocking variant of [`Self::set_config_variable`].
    pub fn set_config_variable_sync(
        &self,
        variable: &str,
        value: &str,
    ) -> Result<(), CommandError> {
        check_hyprland_response(
            self.send_raw_message_sync(format!("keyword {} {}", variable, value).as_str()),
        )
    }
}

fn get_batch_from_recipe(recipe: &[Command]) -> String {
    let mut full_command = String::from("/[[BATCH]]");
    for command in recipe {
        full_command.push_str(command.get_command());
        full_command.push(';');
    }

    full_command
}

fn check_hyprland_response(resp: Result<String, io::Error>) -> Result<(), CommandError> {
    match resp {
        Ok(resp) if resp == "ok" => Ok(()),
        Ok(e) => Err(CommandError::HyprlandError(HyprlandError::new(e))),
        Err(e) => Err(CommandError::IOError(e)),
    }
}
