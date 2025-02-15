use super::*;
use crate::ctl::data::FullscreenState;

pub trait Command {
    fn get_command(&self) -> String;
}
pub trait DispatchCommand: Command {}

macro_rules! make_command {
    ($name:ident, $strname:expr, $($field:ident: $type:ty),*) => {
        make_command!($name, $strname, " ", $($field: $type),*);
    };
    (with_new, $name:ident, $strname:expr, $($field:ident: $type:ty),*) => {
        make_command!(with_new, $name, $strname, " ", $($field: $type),*);
    };
    ($name:ident, $strname:expr, $sep:expr, $($field:ident: $type:ty),*) => {
        #[derive(Debug)]
        pub struct $name {
            $($field: $type),*
        }
        impl Command for $name {
            fn get_command(&self) -> String {
                let args_string = "".to_string() $(+ self.$field.to_string().as_str() + $sep)*;
                format!(concat!($strname, " {}"), &args_string.as_str()[..args_string.len() - $sep.len()])
            }
        }
    };
    (with_new, $name:ident, $strname:expr, $sep:expr, $($field:ident: $type:ty),*) => {
        make_command!($name, $strname, $sep, $($field: $type),*);
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
macro_rules! make_dispatch {
    ($name:ident) => {
        impl DispatchCommand for $name {}
    };
}

// NOTE: For information on the commands, please visit the hyprland wiki
// There are some slight variations, but the naming is approximately the same

// TODO: implement exec
make_command!(with_new, Pass, "pass", to: WindowArgument);
impl DispatchCommand for Pass {}

make_command!(with_new, SendShortcut, "sendshortcut", modifier: ModArgument, key: KeyArgument, window: WindowArgument);
make_dispatch!(SendShortcut);

make_command!(KillActive, "killactive");
make_dispatch!(KillActive);

make_command!(ForceKillActive, "forcekillactive");
make_dispatch!(ForceKillActive);

make_command!(with_new, CloseWindow, "closewindow", which: WindowArgument);
make_dispatch!(CloseWindow);

make_command!(with_new, KillWindow, "killwindow", which: WindowArgument);
make_dispatch!(KillWindow);

// TODO: implement signal

make_command!(with_new, GoToWorkSpace, "workspace", to: WorkspaceArgument);
make_dispatch!(GoToWorkSpace);

make_command!(with_new, MoveToWorkspace, "movetoworkspace", to: WorkspaceArgument, which: WindowArgument);
make_dispatch!(MoveToWorkspace);

make_command!(with_new, MoveToWorkspaceSilent, "movetoworkspacesilent", to: WorkspaceArgument, which: WindowArgument);
make_dispatch!(MoveToWorkspaceSilent);

make_command!(with_new, ToggleFloating, "togglefloating", which: WindowArgument);
make_dispatch!(ToggleFloating);

make_command!(with_new, SetFloating, "setfloating", which: WindowArgument);
make_dispatch!(SetFloating);

make_command!(with_new, SetTiled, "settiled", which: WindowArgument);
make_dispatch!(SetTiled);

make_command!(Fullscreen, "fullscreen 0");
make_dispatch!(Fullscreen);

make_command!(FullscreenMaximize, "fullscreen 1");
make_dispatch!(FullscreenMaximize);

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
make_command!(with_new, SetFullscreenState, "fullscreenstate", internal: FullscreenState, client: FullscreenState);
make_dispatch!(SetFullscreenState);

// TODO: implement dpms

make_command!(with_new, PinWindow, "pin", which: WindowArgument);
make_dispatch!(PinWindow);

make_command!(with_new, MoveFocus, "movefocus", to: DirectionArgument);
make_dispatch!(MoveFocus);

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
make_dispatch!(MoveWindow);
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

make_command!(with_new, SwapWindow, "swapwindow", to: DirectionArgument);
make_dispatch!(SwapWindow);

make_command!(with_new, CenterWindow, "centerwindow", with_exclude: BoolArgument);
make_dispatch!(CenterWindow);

make_command!(with_new, ResizeActiveWindow, "resizeactive", to: ResizeArgument);
make_dispatch!(ResizeActiveWindow);

make_command!(with_new, MoveActiveWindow, "moveactive", to: ResizeArgument);
make_dispatch!(MoveActiveWindow);

make_command!(with_new, ResizeWindow, "resizewindowpixel", ",", to: ResizeArgument, which: WindowArgument);
make_dispatch!(ResizeWindow);

make_command!(with_new, MoveWindowBy, "movewindowpixel", by: ResizeArgument, which: WindowArgument);
make_dispatch!(MoveWindowBy);

make_command!(with_new, CycleNext, "cyclenext", options: CycleNextArguments);
make_dispatch!(CycleNext);

make_command!(SwapWithNext, "swapnext");
make_dispatch!(SwapWithNext);

make_command!(SwapWithPrev, "swapnext prev");
make_dispatch!(SwapWithPrev);

make_command!(with_new, TagWindow, "tagwindow", tag: TagArgument, which: WindowArgument);
make_dispatch!(TagWindow);

make_command!(with_new, FocusWindow, "focuswindow", which: WindowArgument);
make_dispatch!(FocusWindow);

make_command!(with_new, FocusMonitor, "focusmonitor", which: MonitorArgument);
make_dispatch!(FocusMonitor);

make_command!(with_new, SetSplitRatio, "splitratio", split: FloatArgument);
make_dispatch!(SetSplitRatio);
