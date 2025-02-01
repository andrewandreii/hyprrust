use nix::unistd::getuid;
use std::fs::read_dir;
use std::{env, io, path::PathBuf};
use tokio::task::AbortHandle;

pub struct HyprlandConnection {
    instance: String,
    pub(crate) event_handle: Option<AbortHandle>,
    pub(crate) ctl_handle: Option<AbortHandle>,
}

impl HyprlandConnection {
    pub fn new() -> HyprlandConnection {
        HyprlandConnection::new_with_instance(HyprlandConnection::get_current_instance())
    }

    pub fn new_with_instance(instance: String) -> HyprlandConnection {
        HyprlandConnection {
            instance,
            event_handle: None,
            ctl_handle: None,
        }
    }

    pub fn get_current_instance() -> String {
        env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .expect("HYPRLAND_INSTANCE_SIGNATURE not set. Is hyprland running?")
    }

    pub fn get_runtime_dir() -> PathBuf {
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

    pub fn get_instances() -> Vec<String> {
        read_dir(HyprlandConnection::get_runtime_dir())
            .unwrap()
            .map(|inst_dir| inst_dir.unwrap().file_name().into_string().unwrap())
            .collect()
    }

    pub fn get_socket_path(&self) -> Result<PathBuf, io::Error> {
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
