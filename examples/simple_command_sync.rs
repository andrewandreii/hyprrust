use std::error::Error;

use hyprrust::commands::prelude::*;
use hyprrust::HyprlandConnection;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = HyprlandConnection::current().unwrap();

    let command = set_floating(WindowArgument::Tiled);

    conn.send_command_sync(&command)?;
    println!("sucessful");

    Ok(())
}
