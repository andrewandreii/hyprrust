use nix::unistd::getuid;
use std::env::VarError;
use std::fs::read_dir;
use std::{env, io, path::PathBuf};

#[cfg(feature = "async")]
use tokio::task::AbortHandle;

/// Represents a connection to Hyprland, it can be used to start an event listener or to send
/// commands to Hyprland
#[derive(Debug)]
pub struct HyprlandConnection {
    instance: String,
    #[cfg(feature = "async")]
    pub(crate) event_handle: Option<AbortHandle>,
}

impl HyprlandConnection {
    /// Creates a new `HyprlandConnection` object using the current running instance
    pub fn current() -> Result<HyprlandConnection, VarError> {
        Ok(HyprlandConnection::new(
            HyprlandConnection::get_current_instance()?,
        ))
    }

    /// Creates a new instance with the specified instance. Does not check if instance is valid
    pub fn new(instance: String) -> HyprlandConnection {
        HyprlandConnection {
            instance,
            #[cfg(feature = "async")]
            event_handle: None,
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
            PathBuf::from(format!("/run/user/{}/hypr", uid))
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

    pub(crate) fn get_socket_path(&self, socket_name: &str) -> Result<PathBuf, io::Error> {
        let mut path = HyprlandConnection::get_runtime_dir();
        path.push(self.instance.clone());
        path.push(socket_name);

        if path.exists() {
            Ok(path)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Socket file not found.",
            ))
        }
    }

    #[allow(clippy::needless_question_mark)]
    pub(crate) fn get_event_socket_path(&self) -> Result<PathBuf, io::Error> {
        Ok(self.get_socket_path(".socket2.sock")?)
    }

    #[allow(clippy::needless_question_mark)]
    pub(crate) fn get_ctl_socket_path(&self) -> Result<PathBuf, io::Error> {
        Ok(self.get_socket_path(".socket.sock")?)
    }
}
