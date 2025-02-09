use std::io;

use hyprrust::ctl::data::*;
use hyprrust::HyprlandConnection;

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
            .iter()
            .map(|deco| deco.decoration_name.as_str())
            .collect::<Vec<&str>>()
    );

    Ok(())
}
