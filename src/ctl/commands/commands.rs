use super::*;
use crate::ctl::data::FullscreenState;

pub enum CommandType {
    DispatchCommand,
    DirectCommand,
}
pub trait Command {
    fn get_command(&self) -> String;
    fn get_type(&self) -> CommandType;
}

macro_rules! make_command_with_new {
    ($name:ident: $cmd_type:ident, $strname:expr, $($field:ident: $type:ty),*) => {
        make_command_with_new!($name: $cmd_type, $strname, " ", $($field: $type),*);
    };
    ($name:ident: $cmd_type:ident, $strname:expr, $sep:expr, $($field:ident: $type:ty),*) => {
        make_command!($name: $cmd_type, $strname, $sep, $($field: $type),*);
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
    ($name:ident: $cmd_type:ident, $strname:expr, $($field:ident: $type:ty),*) => {
        make_command!($name: $cmd_type, $strname, " ", $($field: $type),*);
    };
    ($name:ident: $cmd_type:ident, $strname:expr, $sep:expr, $($field:ident: $type:ty),*) => {
        #[derive(Debug)]
        pub struct $name {
            $($field: $type),*
        }
        impl Command for $name {
            fn get_command(&self) -> String {
                let args_string = "".to_string() $(+ self.$field.to_string().as_str() + $sep)*;
                format!(concat!($strname, " {}"), &args_string.as_str()[..args_string.len() - $sep.len()])
            }

            fn get_type(&self) -> CommandType {
                CommandType::$cmd_type
            }
        }
    };
    ($name:ident: $type:ident, $strname:expr) => {
        pub struct $name;
        impl Command for $name {
            fn get_command(&self) -> String {
                $strname.to_string()
            }

            fn get_type(&self) -> CommandType {
                CommandType::$type
            }
        }
    };
}

// NOTE: For information on the commands, please visit the hyprland wiki
// There are some slight variations, but the naming is approximately the same

// TODO: implement exec
make_command_with_new!(Pass: DispatchCommand, "pass", to: WindowArgument);

make_command_with_new!(SendShortcut: DispatchCommand, "sendshortcut", modifier: ModArgument, key: KeyArgument, window: WindowArgument);

make_command!(KillActive: DispatchCommand, "killactive");

make_command!(ForceKillActive: DispatchCommand, "forcekillactive");

make_command_with_new!(CloseWindow: DispatchCommand, "closewindow", which: WindowArgument);

make_command_with_new!(KillWindow: DispatchCommand, "killwindow", which: WindowArgument);

// TODO: implement signal

make_command_with_new!(GoToWorkSpace: DispatchCommand, "workspace", to: WorkspaceArgument);

make_command_with_new!(MoveToWorkspace: DispatchCommand, "movetoworkspace", to: WorkspaceArgument, which: WindowArgument);

make_command_with_new!(MoveToWorkspaceSilent: DispatchCommand, "movetoworkspacesilent", to: WorkspaceArgument, which: WindowArgument);

make_command_with_new!(ToggleFloating: DispatchCommand, "togglefloating", which: WindowArgument);

make_command_with_new!(SetFloating: DispatchCommand, "setfloating", which: WindowArgument);

make_command_with_new!(SetTiled: DispatchCommand, "settiled", which: WindowArgument);

make_command!(Fullscreen: DispatchCommand, "fullscreen 0");

make_command!(FullscreenMaximize: DispatchCommand, "fullscreen 1");

impl ToString for FullscreenState {
    fn to_string(&self) -> String {
        match self {
            FullscreenState::None => "0",
            FullscreenState::Maximized => "1",
            FullscreenState::Fullscreen => "2",
            FullscreenState::Max => "3",
        }
        .to_string()
    }
}
make_command_with_new!(SetFullscreenState: DispatchCommand, "fullscreenstate", internal: FullscreenState, client: FullscreenState);

// TODO: implement dpms

make_command_with_new!(PinWindow: DispatchCommand, "pin", which: WindowArgument);

make_command_with_new!(MoveFocus: DispatchCommand, "movefocus", to: DirectionArgument);

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
            Either::First(dir) => format!("movewindow {} {}", dir.to_string(), silent_str),
            Either::Second(mon) => format!("movewindow mon:{} {}", mon.to_string(), silent_str),
        }
    }

    fn get_type(&self) -> CommandType {
        CommandType::DispatchCommand
    }
}

make_command_with_new!(SwapWindow: DispatchCommand, "swapwindow", to: DirectionArgument);

make_command_with_new!(CenterWindow: DispatchCommand, "centerwindow", with_exclude: BoolArgument);

make_command_with_new!(ResizeActiveWindow: DispatchCommand, "resizeactive", to: ResizeArgument);

make_command_with_new!(MoveActiveWindow: DispatchCommand, "moveactive", to: ResizeArgument);

make_command_with_new!(ResizeWindow: DispatchCommand, "resizewindowpixel", ",", to: ResizeArgument, which: WindowArgument);

make_command_with_new!(MoveWindowBy: DispatchCommand, "movewindowpixel", ",", by: ResizeArgument, which: WindowArgument);

make_command_with_new!(CycleNext: DispatchCommand, "cyclenext", options: CycleNextArguments);

make_command!(SwapWithNext: DispatchCommand, "swapnext");

make_command!(SwapWithPrev: DispatchCommand, "swapnext prev");

make_command_with_new!(TagWindow: DispatchCommand, "tagwindow", tag: TagArgument, which: WindowArgument);

make_command_with_new!(FocusWindow: DispatchCommand, "focuswindow", which: WindowArgument);

make_command_with_new!(FocusMonitor: DispatchCommand, "focusmonitor", which: MonitorArgument);

make_command_with_new!(SetSplitRatio: DispatchCommand, "splitratio", split: FloatArgument);

make_command_with_new!(MoveCursorToCorner: DispatchCommand, "movecursortocorner", which: CornerArgument);

make_command_with_new!(MoveCursor: DispatchCommand, "movecursor", x: IntArgument, y: IntArgument);

make_command_with_new!(RenameWorkspace: DispatchCommand, "renameworkspace", which_id: StringArgument, new_name: StringArgument);

make_command_with_new!(ExitHyprland: DispatchCommand, "exit",);

make_command_with_new!(ForceRenderReload: DispatchCommand, "forcerenderreload",);
