use std::error::Error;

use hyprrust::ctl::command::*;
use hyprrust::HyprlandConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn = HyprlandConnection::new();

    let command = SetFloating::new(WindowArgument::Tiled);

    conn.send_command(&command).await?;
    println!("sucessful");

    Ok(())
}
