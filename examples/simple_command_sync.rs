use std::error::Error;

use hyprrust::commands::prelude::*;
use hyprrust::HyprlandConnection;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = HyprlandConnection::new();

    let command = SetFloating::new(WindowArgument::Tiled);

    conn.send_command_sync(&command)?;
    println!("sucessful");

    Ok(())
}
