#![allow(clippy::new_without_default)]

use super::*;
use crate::ctl::data::FullscreenState;

pub trait Command {
    fn get_command(&self) -> String;
}

macro_rules! make_command_with_new {
    ($name:ident, $strname:expr, $($field:ident: $type:ty),*) => {
        make_command_with_new!($name, $strname, " ", $($field: $type),*);
    };
    ($name:ident, $strname:expr, $sep:expr, $($field:ident: $type:ty),*) => {
        make_command!($name, $strname, $sep, $($field: $type),*);
        impl $name {
            pub fn new($($field: $type),*) -> Self {
                $name {
                    $($field),*
                }
            }
        }
    };
}

macro_rules! make_command {
    ($name:ident, $strname:expr, $($field:ident: $type:ty),*) => {
        make_command!($name, $strname, " ", $($field: $type),*);
    };
    ($name:ident, $strname:expr, $sep:expr, $($field:ident: $type:ty),*) => {
        #[derive(Debug)]
        pub struct $name {
            $($field: $type),*
        }
        impl Command for $name {
            fn get_command(&self) -> String {
                let args_string = "".to_string() $(+ self.$field.to_argument_string().as_str() + $sep)*;
                format!(concat!($strname, " {}"), &args_string.as_str()[..args_string.len() - $sep.len()])
            }
        }
    };
    ($name:ident, $strname:expr) => {
        pub struct $name;
        impl Command for $name {
            fn get_command(&self) -> String {
                $strname.to_string()
            }
        }
    };
}

// NOTE: For information on the commands, please visit the hyprland wiki
// There are some slight variations, but the naming is approximately the same

// TODO: implement exec
make_command_with_new!(Pass, "dispatch pass", to: WindowArgument);

make_command_with_new!(SendShortcut, "dispatch sendshortcut", modifier: ModArgument, key: KeyArgument, window: WindowArgument);

make_command!(KillActive, "dispatch killactive");

make_command!(ForceKillActive, "dispatch forcekillactive");

make_command_with_new!(CloseWindow, "dispatch closewindow", which: WindowArgument);

make_command_with_new!(KillWindow, "dispatch killwindow", which: WindowArgument);

// TODO: implement signal

make_command_with_new!(GoToWorkSpace, "dispatch workspace", to: WorkspaceArgument);

make_command_with_new!(MoveToWorkspace, "dispatch movetoworkspace", to: WorkspaceArgument, which: WindowArgument);

make_command_with_new!(MoveToWorkspaceSilent, "dispatch movetoworkspacesilent", to: WorkspaceArgument, which: WindowArgument);

make_command_with_new!(ToggleFloating, "dispatch togglefloating", which: WindowArgument);

make_command_with_new!(SetFloating, "dispatch setfloating", which: WindowArgument);

make_command_with_new!(SetTiled, "dispatch settiled", which: WindowArgument);

make_command!(Fullscreen, "dispatch fullscreen 0");

make_command!(FullscreenMaximize, "dispatch fullscreen 1");

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
make_command_with_new!(SetFullscreenState, "dispatch fullscreenstate", internal: FullscreenState, client: FullscreenState);

// TODO: implement dpms

make_command_with_new!(PinWindow, "dispatch pin", which: WindowArgument);

make_command_with_new!(MoveFocus, "dispatch movefocus", to: DirectionArgument);

#[derive(Debug)]
pub enum Either<T, U> {
    First(T),
    Second(U),
}

#[derive(Debug)]
pub struct MoveWindow {
    what: Either<DirectionArgument, MonitorArgument>,
    silent: bool,
}

impl MoveWindow {
    pub fn with_direction(direction: DirectionArgument, silent: bool) -> Self {
        MoveWindow {
            what: Either::First(direction),
            silent,
        }
    }

    pub fn with_monitor(monitor: MonitorArgument, silent: bool) -> Self {
        MoveWindow {
            what: Either::Second(monitor),
            silent,
        }
    }
}
impl Command for MoveWindow {
    fn get_command(&self) -> String {
        let silent_str = if self.silent { "silent" } else { "" };
        match &self.what {
            Either::First(dir) => format!("movewindow {} {}", dir.to_argument_string(), silent_str),
            Either::Second(mon) => {
                format!(
                    "dispatch movewindow mon:{} {}",
                    mon.to_argument_string(),
                    silent_str
                )
            }
        }
    }
}

make_command_with_new!(SwapWindow, "dispatch swapwindow", to: DirectionArgument);

make_command_with_new!(CenterWindow, "dispatch centerwindow", with_exclude: BoolArgument);

make_command_with_new!(ResizeActiveWindow, "dispatch resizeactive", to: ResizeArgument);

make_command_with_new!(MoveActiveWindow, "dispatch moveactive", to: ResizeArgument);

make_command_with_new!(ResizeWindow, "dispatch resizewindowpixel", ",", to: ResizeArgument, which: WindowArgument);

make_command_with_new!(MoveWindowBy, "dispatch movewindowpixel", ",", by: ResizeArgument, which: WindowArgument);

make_command_with_new!(CycleNext, "dispatch cyclenext", options: CycleNextArguments);

make_command!(SwapWithNext, "dispatch swapnext");

make_command!(SwapWithPrev, "dispatch swapnext prev");

make_command_with_new!(TagWindow, "dispatch tagwindow", tag: TagArgument, which: WindowArgument);

make_command_with_new!(FocusWindow, "dispatch focuswindow", which: WindowArgument);

make_command_with_new!(FocusMonitor, "dispatch focusmonitor", which: MonitorArgument);

make_command_with_new!(SetSplitRatio, "dispatch splitratio", split: FloatArgument);

make_command_with_new!(MoveCursorToCorner, "dispatch movecursortocorner", which: CornerArgument);

make_command_with_new!(MoveCursor, "dispatch movecursor", x: IntArgument, y: IntArgument);

make_command_with_new!(RenameWorkspace, "dispatch renameworkspace", which_id: StringArgument, new_name: StringArgument);

make_command_with_new!(ExitHyprland, "dispatch exit",);

make_command_with_new!(ForceRenderReload, "dispatch forcerenderreload",);

pub struct SetConfigValue {
    variable: StringArgument,
    value: StringArgument,
}

impl SetConfigValue {
    pub fn new(variable: StringArgument, value: StringArgument) -> Self {
        SetConfigValue { variable, value }
    }
}

impl Command for SetConfigValue {
    fn get_command(&self) -> String {
        format!(
            "keyword {} {}",
            self.variable.to_argument_string(),
            self.value.to_argument_string()
        )
    }
}

make_command_with_new!(ReloadConfig, "reload",);
make_command_with_new!(KillWindowMode, "kill",);
make_command_with_new!(SetCursorTheme, "setcursor", theme_name: StringArgument, size: IntArgument);
make_command_with_new!(SetError, "seterror", color: ColorArgument, message: StringArgument);
make_command_with_new!(Notify, "notify", icon: NotifyIconArgument, time_ms: IntArgument, color: ColorArgument, message: StringArgument);
// TODO: implement output, setxkblayout, dismissnotify
