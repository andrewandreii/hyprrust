#![allow(clippy::new_without_default)]

use std::borrow::Cow;

use super::*;
use crate::ctl::data::FullscreenState;

pub struct Command(Cow<'static, str>);
impl Command {
    pub fn new(cmd: Cow<'static, str>) -> Self {
        Command(cmd)
    }

    pub fn get_command(&self) -> &str {
        &self.0
    }
}

/// Used to easily create commands.
///
/// ```rust
/// use hyprrust::HyprlandConnection;
/// use hyprrust::commands::prelude::*;
///
/// make_command!(execr, "dispatch execr", cmd: String);
///
/// let conn = HyprlandConnection::new();
/// conn.send_command_sync(&execr("kitty".to_string())).unwrap();
/// ```
#[macro_export]
macro_rules! make_command {
    ($name:ident, $strname:expr, $($field:ident: $type:ty),*) => {
        make_command!($name, $strname, " ", $($field: $type),*);
    };
    ($name:ident, $strname:expr, $sep:expr, $($field:ident: $type:ty),*) => {
        pub fn $name($($field: $type),*) -> Command {
            use std::borrow::Cow;
            let args_string = "".to_string() $(+ $field.to_argument_string().as_str() + $sep)*;
            let cmd = format!(concat!($strname, " {}"), &args_string.as_str()[..args_string.len() - $sep.len()]);
            Command::new(Cow::Owned(cmd))
        }
    };
    ($name:ident, $strname:expr) => {
        pub fn $name() -> Command {
            use std::borrow::Cow;
            Command::new(Cow::Borrowed($strname))
        }
    };
}
pub use make_command;

// NOTE: For information on the commands, please visit the hyprland wiki
// There are some slight variations, but the naming is approximately the same

// TODO: implement exec
make_command!(pass, "dispatch pass", to: WindowArgument);
make_command!(send_shortcut, "dispatch sendshortcut", modifier: ModArgument, key: KeyArgument, window: WindowArgument);
make_command!(kill_active, "dispatch killactive");
make_command!(force_kill_active, "dispatch forcekillactive");
make_command!(close_window, "dispatch closewindow", which: WindowArgument);
make_command!(kill_window, "dispatch killwindow", which: WindowArgument);
// TODO: implement signal
make_command!(go_to_work_space, "dispatch workspace", to: WorkspaceArgument);
make_command!(move_to_workspace, "dispatch movetoworkspace", to: WorkspaceArgument, which: WindowArgument);
make_command!(move_to_workspace_silent, "dispatch movetoworkspacesilent", to: WorkspaceArgument, which: WindowArgument);
make_command!(toggle_floating, "dispatch togglefloating", which: WindowArgument);
make_command!(set_floating, "dispatch setfloating", which: WindowArgument);
make_command!(set_tiled, "dispatch settiled", which: WindowArgument);
make_command!(fullscreen, "dispatch fullscreen 0");
make_command!(fullscreen_maximize, "dispatch fullscreen 1");

impl Argument for FullscreenState {
    fn to_argument_string(&self) -> String {
        match self {
            FullscreenState::None => "0",
            FullscreenState::Maximized => "1",
            FullscreenState::Fullscreen => "2",
            FullscreenState::Max => "3",
        }
        .to_string()
    }
}

make_command!(set_fullscreen_state, "dispatch fullscreenstate", internal: FullscreenState, client: FullscreenState);
// TODO: implement dpms
make_command!(pin_window, "dispatch pin", which: WindowArgument);
make_command!(move_focus, "dispatch movefocus", to: DirectionArgument);

pub fn move_window_in_direction(direction: DirectionArgument, silent: bool) -> Command {
    Command(Cow::Owned(format!(
        "dispatch movewindow {} {}",
        direction.to_argument_string(),
        if silent { "silent" } else { "" }
    )))
}

pub fn move_window_to_monitor(monitor: MonitorArgument, silent: bool) -> Command {
    Command(Cow::Owned(format!(
        "dispatch movewindow {} {}",
        monitor.to_argument_string(),
        if silent { "silent" } else { "" }
    )))
}

make_command!(swap_window, "dispatch swapwindow", to: DirectionArgument);
make_command!(center_window, "dispatch centerwindow", with_exclude: bool);
make_command!(resize_active_window, "dispatch resizeactive", to: ResizeArgument);
make_command!(move_active_window, "dispatch moveactive", to: ResizeArgument);
make_command!(resize_window, "dispatch resizewindowpixel", ",", to: ResizeArgument, which: WindowArgument);
make_command!(move_window_by, "dispatch movewindowpixel", ",", by: ResizeArgument, which: WindowArgument);
make_command!(cycle_next, "dispatch cyclenext", options: CycleNextArguments);
make_command!(swap_with_next, "dispatch swapnext");
make_command!(swap_with_prev, "dispatch swapnext prev");
make_command!(tag_window, "dispatch tagwindow", tag: TagArgument, which: WindowArgument);
make_command!(focus_window, "dispatch focuswindow", which: WindowArgument);
make_command!(focus_monitor, "dispatch focusmonitor", which: MonitorArgument);
make_command!(set_split_ratio, "dispatch splitratio", split: FloatArgument);
make_command!(move_cursor_to_corner, "dispatch movecursortocorner", which: CornerArgument);
make_command!(move_cursor, "dispatch movecursor", x: i32, y: i32);
make_command!(rename_workspace, "dispatch renameworkspace", which_id: String, new_name: String);
make_command!(exit_hyprland, "dispatch exit",);
make_command!(force_render_reload, "dispatch forcerenderreload",);
make_command!(set_config_value, "", variable: String, value: String);
make_command!(reload_config, "reload",);
make_command!(kill_window_mode, "kill",);
make_command!(set_cursor_theme, "setcursor", theme_name: String, size: i32);
make_command!(set_error, "seterror", color: ColorArgument, message: String);
make_command!(notify, "notify", icon: NotifyIconArgument, time_ms: i32, color: ColorArgument, message: String);
// TODO: implement output, setxkblayout, dismissnotify
