use super::*;
use crate::ctl::data::FullscreenState;

pub trait Command {
    fn get_command(&self) -> String;
}
pub trait DispatchCommand: Command {}

macro_rules! make_command {
    ($name:ident, $strname:expr, $($field:ident: $type:ty),*) => {
        #[derive(Debug)]
        pub struct $name {
            $($field: $type),*
        }
        impl Command for $name {
            fn get_command(&self) -> String {
                format!(concat!($strname, " {}"), "".to_string() $(+ self.$field.to_string().as_str())*)
            }
        }
    };
    (with_new, $name:ident, $strname:expr, $($field:ident: $type:ty),*) => {
        make_command!($name, $strname, $($field: $type),*);
        impl $name {
            pub fn new($($field: $type),*) -> Self {
                $name {
                    $($field),*
                }
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
make_command!(with_new, Pass, "pass", to: WindowArgument);
impl DispatchCommand for Pass {}

make_command!(with_new, SendShortcut, "sendshortcut", modifier: ModArgument, key: KeyArgument, window: WindowArgument);
impl DispatchCommand for SendShortcut {}

make_command!(KillActive, "killactive");
impl DispatchCommand for KillActive {}

make_command!(ForceKillActive, "forcekillactive");
impl DispatchCommand for ForceKillActive {}

make_command!(with_new, CloseWindow, "closewindow", which: WindowArgument);
impl DispatchCommand for CloseWindow {}

make_command!(with_new, KillWindow, "killwindow", which: WindowArgument);
impl DispatchCommand for KillWindow {}

// TODO: implement signal

make_command!(with_new, GoToWorkSpace, "workspace", to: WorkspaceArgument);
impl DispatchCommand for GoToWorkSpace {}

make_command!(with_new, MoveToWorkspace, "movetoworkspace", to: WorkspaceArgument, which: WindowArgument);
impl DispatchCommand for MoveToWorkspace {}

make_command!(with_new, MoveToWorkspaceSilent, "movetoworkspacesilent", to: WorkspaceArgument, which: WindowArgument);
impl DispatchCommand for MoveToWorkspaceSilent {}

make_command!(with_new, ToggleFloating, "togglefloating", which: WindowArgument);
impl DispatchCommand for ToggleFloating {}

make_command!(with_new, SetFloating, "setfloating", which: WindowArgument);
impl DispatchCommand for SetFloating {}

make_command!(with_new, SetTiled, "settiled", which: WindowArgument);
impl DispatchCommand for SetTiled {}

make_command!(Fullscreen, "fullscreen 0");
impl DispatchCommand for Fullscreen {}

make_command!(FullscreenMaximize, "fullscreen 1");
impl DispatchCommand for FullscreenMaximize {}

// TODO: should be newtype?
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
make_command!(SetFullscreenState, "fullscreenstate", internal: FullscreenState, client: FullscreenState);
impl DispatchCommand for SetFullscreenState {}

// TODO: implement dpms

make_command!(with_new, PinWindow, "pin", which: WindowArgument);
impl DispatchCommand for PinWindow {}

make_command!(with_new, MoveFocus, "movefocus", to: DirectionArgument);
impl DispatchCommand for MoveFocus {}

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
impl DispatchCommand for MoveWindow {}
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
}

make_command!(with_new, ResizeActiveWindow, "resizeactive", to: ResizeArgument);
impl DispatchCommand for ResizeActiveWindow {}
