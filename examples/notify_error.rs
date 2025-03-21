use hyprrust::{
    arguments::{ColorArgument, NotifyIconArgument, WindowArgument},
    commands::*,
    data::Window,
    HyprlandConnection,
};

fn main() {
    let conn = HyprlandConnection::current().unwrap();

    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Not enough arguments!");
        return;
    }
    let fav_window: String = args[1].to_string();

    conn.send_command_sync(&focus_window(WindowArgument::Class(fav_window.clone())))
        .unwrap();

    if conn.get_sync::<Window>().unwrap().class != fav_window {
        conn.send_command_sync(&set_error(
            ColorArgument::new(255, 40, 40, 255),
            "Could not focus your favourite window.".to_string(),
        ))
        .unwrap();
    } else {
        conn.send_command_sync(&notify(
            NotifyIconArgument::Ok,
            10000,
            ColorArgument::new(40, 200, 120, 255),
            "Everything went well!".to_string(),
        ))
        .unwrap();
    }
}
