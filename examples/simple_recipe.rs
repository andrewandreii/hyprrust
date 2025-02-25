use hyprrust::ctl::command::*;
use hyprrust::HyprlandConnection;

#[tokio::main]
async fn main() {
    let conn = HyprlandConnection::new();

    // make the current window 400x400, place it in the top right corner and pin it
    let commands: Recipe = recipe![
        SetFloating::new(WindowArgument::ActiveWindow),
        ResizeActiveWindow::new(ResizeArgument::Exact(
            NumPercent::Number(400),
            NumPercent::Number(400),
        )),
        MoveWindow::with_direction(DirectionArgument::Up, true),
        MoveWindow::with_direction(DirectionArgument::Right, true),
        PinWindow::new(WindowArgument::ActiveWindow),
    ];

    match conn.send_recipe(&commands).await {
        Ok(()) => println!("successful"),
        Err(errors) => println!("{:?}", errors),
    }
}
