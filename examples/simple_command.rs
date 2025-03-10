use std::error::Error;

use hyprrust::commands::prelude::*;
use hyprrust::HyprlandConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn = HyprlandConnection::new();

    let command = set_floating(WindowArgument::Tiled);

    conn.send_command(&command).await?;
    println!("sucessful");

    Ok(())
}
