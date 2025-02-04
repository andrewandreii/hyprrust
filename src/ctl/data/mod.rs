pub mod data_models;
mod deserializing;
pub mod prelude;

use crate::connection::HyprlandConnection;
use deserializing::{HyprlandData, HyprlandDataWithArgument};
use serde::de::DeserializeOwned;

use std::io;

impl HyprlandConnection {
    pub async fn get<T>(&self) -> Result<T, io::Error>
    where
        T: HyprlandData + DeserializeOwned,
    {
        let command = format!("-j/{}", T::get_command());
        let resp = self.send_raw_message(command.as_str()).await?;

        Ok(serde_json::from_str(resp.as_str())?)
    }

    pub async fn get_with_argument<T>(&self, arg: String) -> Result<T, io::Error>
    where
        T: HyprlandDataWithArgument + DeserializeOwned,
    {
        let command = format!("-j/{}", T::get_command(arg));
        let resp = self.send_raw_message(command.as_str()).await?;

        Ok(serde_json::from_str(resp.as_str())?)
    }

    pub fn get_sync<T>(&self) -> Result<T, io::Error>
    where
        T: HyprlandData + DeserializeOwned,
    {
        let command = format!("-j/{}", T::get_command());
        let resp = self.send_raw_message_sync(command.as_str())?;

        Ok(serde_json::from_str(resp.as_str())?)
    }

    pub fn get_with_argument_sync<T>(&self, arg: String) -> Result<T, io::Error>
    where
        T: HyprlandDataWithArgument + DeserializeOwned,
    {
        let command = format!("-j/{}", T::get_command(arg));
        let resp = self.send_raw_message_sync(command.as_str())?;

        Ok(serde_json::from_str(resp.as_str())?)
    }
}
