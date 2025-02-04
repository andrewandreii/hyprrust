use std::io;

use hyprrust::connection::HyprlandConnection;
use hyprrust::ctl::data::prelude::*;

fn main() -> Result<(), io::Error> {
    let conn = HyprlandConnection::new();

    println!(
        "Hyprland version: {:?}",
        conn.get_sync::<Version>()?.version
    );

    let current_win = conn.get_sync::<Window>()?;
    println!("Current window title: {:?}", current_win.title);

    println!(
        "Decorations active for current window: {:?}",
        conn.get_with_argument_sync::<Decorations>(format!("address:{}", current_win.address))?
            .decorations
            .iter()
            .map(|deco| deco.decoration_name.as_str())
            .collect::<Vec<&str>>()
    );

    Ok(())
}
