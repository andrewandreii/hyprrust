mod data_models;
pub use data_models::*;
mod deserializing;

use crate::connection::HyprlandConnection;
use deserializing::{HyprlandData, HyprlandDataWithArgument};
use serde::de::DeserializeOwned;

use std::io;

impl HyprlandConnection {
    /// Returns the data T requested from Hyprland.
    ///
    /// # Example
    ///
    /// use hyprrust::connection::HyprlandConnection;
    /// use hyprrust::ctl::data::*;
    /// let conn = HyprlandConnection::new();
    /// let version = conn.get<Version>().await?;
    pub async fn get<T>(&self) -> Result<T, io::Error>
    where
        T: HyprlandData + DeserializeOwned,
    {
        let command = format!("-j/{}", T::get_command());
        let resp = self.send_raw_message(command.as_str()).await?;

        Ok(serde_json::from_str(resp.as_str())?)
    }

    // TODO: The argument should be of a type from arguments.rs
    /// Returns the data T requested from Hyprland also passing an argument
    ///
    /// # Example
    ///
    /// use hyprrust::connection::HyprlandConnection;
    /// use hyprrust::ctl::data::*;
    /// let conn = HyprlandConnection::new();
    /// let terminal_decorations = conn.get<Decorations>("class:st".to_string()).await?;
    pub async fn get_with_argument<T>(&self, arg: String) -> Result<T, io::Error>
    where
        T: HyprlandDataWithArgument + DeserializeOwned,
    {
        let command = format!("-j/{}", T::get_command(arg));
        let resp = self.send_raw_message(command.as_str()).await?;

        Ok(serde_json::from_str(resp.as_str())?)
    }

    /// The same behaviour as get, but without async. See [`get`]: #method.get
    pub fn get_sync<T>(&self) -> Result<T, io::Error>
    where
        T: HyprlandData + DeserializeOwned,
    {
        let command = format!("-j/{}", T::get_command());
        let resp = self.send_raw_message_sync(command.as_str())?;

        Ok(serde_json::from_str(resp.as_str())?)
    }

    /// The same behaviour as get_with_argument, but without async.
    /// See [`get_with_argument`]: #method.get_with_argument
    pub fn get_with_argument_sync<T>(&self, arg: String) -> Result<T, io::Error>
    where
        T: HyprlandDataWithArgument + DeserializeOwned,
    {
        let command = format!("-j/{}", T::get_command(arg));
        let resp = self.send_raw_message_sync(command.as_str())?;

        Ok(serde_json::from_str(resp.as_str())?)
    }
}

#[cfg(test)]
mod data_tests {
    use crate::connection::HyprlandConnection;

    #[test]
    fn test_data_models() {
        use super::data_models::*;
        let conn = HyprlandConnection::new();

        assert!(conn.get_sync::<Version>().is_ok());
        assert!(conn.get_sync::<Monitors>().is_ok());
        assert!(conn.get_sync::<Workspace>().is_ok());
        assert!(conn.get_sync::<Workspaces>().is_ok());
        assert!(conn.get_sync::<WorkspaceRules>().is_ok());
        assert!(conn.get_sync::<Window>().is_ok());
        assert!(conn.get_sync::<Clients>().is_ok());
        assert!(conn.get_sync::<Devices>().is_ok());
        assert!(conn
            .get_with_argument_sync::<Decorations>("class:st".to_owned())
            .is_ok());
        assert!(conn.get_sync::<Binds>().is_ok());
        assert!(conn.get_sync::<Layers>().is_ok());
        assert!(conn
            .get_with_argument_sync::<HyprlandOption>("misc:vfr".to_owned())
            .is_ok());
        assert!(conn.get_sync::<CursorPosition>().is_ok());
        assert!(conn.get_sync::<Animations>().is_ok());
        assert!(conn.get_sync::<ConfigErrors>().is_ok());
        assert!(conn.get_sync::<Layouts>().is_ok());
        assert!(conn.get_sync::<Workspace>().is_ok());
    }
}
