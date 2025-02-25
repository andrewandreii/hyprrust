use nix::unistd::getuid;
use std::env::VarError;
use std::fs::read_dir;
use std::{env, io, path::PathBuf};
use tokio::sync::broadcast;
use tokio::task::AbortHandle;

use crate::events::HyprlandEvent;

#[derive(Debug)]
pub(crate) struct EventConnection {
    pub abort_handle: AbortHandle,
    pub receiver: broadcast::Receiver<HyprlandEvent>,
}

/// Represents a connection to Hyprland, it can be used to start an event listener or to send
/// commands to Hyprland
#[derive(Debug)]
pub struct HyprlandConnection {
    instance: String,
    pub(crate) event_connection: Option<EventConnection>,
}

impl HyprlandConnection {
    /// Creates a new `HyprlandConnection` object using the current running instance
    ///
    /// # Panics
    ///
    /// This function panics if it cannot get the current running hyprland instance from the
    /// environmental variable HYPRLAND_INSTANCE_SIGNATURE
    pub fn new() -> HyprlandConnection {
        HyprlandConnection::new_with_instance(
            HyprlandConnection::get_current_instance()
                .expect("HYPRLAND_INSTANCE_SIGNATURE not found. Is Hyprland running?"),
        )
    }

    /// Creates a new instance with the specified instance. Does not check if instance is valid
    pub fn new_with_instance(instance: String) -> HyprlandConnection {
        HyprlandConnection {
            instance,
            event_connection: None,
        }
    }

    /// Returns the current Hyprland instance
    pub fn get_current_instance() -> Result<String, VarError> {
        env::var("HYPRLAND_INSTANCE_SIGNATURE")
    }

    /// Checks if the HYPRLAND_INSTANCE_SIGNATURE environmental variable exists
    pub fn is_hyprland_running() -> bool {
        env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
    }

    fn get_runtime_dir() -> PathBuf {
        if let Ok(dir) = env::var("XDG_RUNTIME_DIR") {
            let mut path = PathBuf::from(dir);
            path.push("hypr");
            path
        } else {
            let uid = getuid();
            let path = PathBuf::from(format!("/run/user/{}/hypr", uid));
            path
        }
    }

    /// Returns a vector of all the Hyprland instances currently running
    pub fn get_instances() -> Result<Vec<String>, io::Error> {
        Ok(read_dir(HyprlandConnection::get_runtime_dir())?
            .filter_map(|inst_dir| {
                if let Ok(dir) = inst_dir {
                    Some(dir.file_name().into_string().unwrap())
                } else {
                    None
                }
            })
            .collect())
    }

    pub(crate) fn get_socket_path(&self) -> Result<PathBuf, io::Error> {
        let mut path = HyprlandConnection::get_runtime_dir();
        path.push(self.instance.clone());

        if path.exists() {
            Ok(path)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Socket file not found.",
            ))
        }
    }
}

impl Default for HyprlandConnection {
    fn default() -> Self {
        Self::new()
    }
}
