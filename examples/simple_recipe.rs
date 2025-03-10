use hyprrust::commands::prelude::*;
use hyprrust::HyprlandConnection;

#[tokio::main]
async fn main() {
    let conn = HyprlandConnection::new();

    // make the current window 400x400, place it in the top right corner and pin it
    let commands = [
        set_floating(WindowArgument::ActiveWindow),
        resize_active_window(ResizeArgument::Exact(
            NumPercent::Number(400),
            NumPercent::Number(400),
        )),
        move_window_in_direction(DirectionArgument::Up, true),
        move_window_in_direction(DirectionArgument::Right, true),
        pin_window(WindowArgument::ActiveWindow),
    ];

    match conn.send_recipe(&commands).await {
        Ok(()) => println!("successful"),
        Err(errors) => println!("{:?}", errors),
    }
}
