use std::ops::Deref;

use hyprrust::connection::HyprlandConnection;
use hyprrust::ctl::arguments::*;
use hyprrust::ctl::command::*;

#[tokio::main]
async fn main() {
    let conn = HyprlandConnection::new();

    // make the current window 400x400, place it in the top right corner and pin it
    let commands: [Box<dyn DispatchCommand>; 5] = [
        Box::new(SetFloating::new(WindowArgument::ActiveWindow)),
        Box::new(ResizeActiveWindow::new(ResizeArgument::Exact(
            NumPercent::Number(400),
            NumPercent::Number(400),
        ))),
        Box::new(MoveWindow::with_direction(DirectionArgument::Up, true)),
        Box::new(MoveWindow::with_direction(DirectionArgument::Right, true)),
        Box::new(PinWindow::new(WindowArgument::ActiveWindow)),
    ];

    for command in commands {
        match conn.dispatch(command.deref()).await {
            Ok(_) => println!("successful"),
            Err(CommandError::IOError(e)) => println!("io error {}", e),
            Err(CommandError::HyprlandError(e)) => println!("{}", e),
        }
    }
}
