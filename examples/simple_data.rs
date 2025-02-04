use std::io;

use hyprrust::connection::HyprlandConnection;
use hyprrust::ctl::data::prelude::*;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let conn = HyprlandConnection::new();

    println!(
        "Hyprland version: {:?}",
        conn.get::<Version>().await?.version
    );

    let current_win = conn.get::<Window>().await?;
    println!("Current window title: {:?}", current_win.title);

    println!(
        "Decorations active for current window: {:?}",
        conn.get_with_argument::<Decorations>(format!("address:{}", current_win.address))
            .await?
            .decorations
            .iter()
            .map(|deco| deco.decoration_name.as_str())
            .collect::<Vec<&str>>()
    );

    Ok(())
}
