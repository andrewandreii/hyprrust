use core::fmt;

#[derive(Debug)]
pub enum WindowArgument {
    Class(String),
    InitialClass(String),
    Title(String),
    InitialTitle(String),
    Tag(String),
    Pid(i32),
    Address(String),
    ActiveWindow,
    Floating,
    Tiled,
}
impl ToString for WindowArgument {
    fn to_string(&self) -> String {
        use WindowArgument::*;
        match self {
            Class(class) => format!("class:{}", class),
            InitialClass(initialclass) => format!("initialclass:{}", initialclass),
            Title(title) => format!("title:{}", title),
            InitialTitle(initialtitle) => format!("initialtitle:{}", initialtitle),
            Tag(tag) => format!("tag:{}", tag),
            Pid(pid) => format!("pid:{}", pid),
            Address(address) => format!("address:{}", address),
            ActiveWindow => "activewindow".to_owned(),
            Floating => "floating".to_owned(),
            Tiled => "tiled".to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum RelAbs {
    Relative(i32),
    Absolute(u32),
}
impl ToString for RelAbs {
    fn to_string(&self) -> String {
        match self {
            RelAbs::Relative(id) if *id >= 0 => format!("+{}", id),
            RelAbs::Relative(id) => id.to_string(),
            RelAbs::Absolute(id) => format!("~{}", id),
        }
    }
}
#[derive(Debug)]
pub enum WorkspaceArgument {
    ID(u64),
    RelativeID(i32),
    WorkspaceOnMonitor(RelAbs),
    WorkspaceOnMonitorWithEmpty(RelAbs),
    OpenWorkspace(RelAbs),
    Name(String),
    Previous,
    PreviousPerMonitor,
    Empty,
    EmptyOnMonitor,
    EmptyNext,
    EmptyNextOnMonitor,
    Special(Option<String>),
}
impl ToString for WorkspaceArgument {
    fn to_string(&self) -> String {
        use WorkspaceArgument::*;
        match self {
            ID(id) => id.to_string(),
            RelativeID(id) => RelAbs::Relative(*id).to_string(),
            WorkspaceOnMonitor(relabs) => format!("m{}", relabs.to_string()),
            WorkspaceOnMonitorWithEmpty(relabs) => format!("r{}", relabs.to_string()),
            OpenWorkspace(relabs) => format!("e{}", relabs.to_string()),
            Name(name) => format!("name:{}", name),
            Previous => "previous".to_string(),
            PreviousPerMonitor => "previous_per_monitor".to_string(),
            Empty => "empty".to_string(),
            EmptyOnMonitor => "emptym".to_string(),
            EmptyNext => "emptyn".to_string(),
            EmptyNextOnMonitor => "emptymn".to_string(),
            Special(None) => "special".to_string(),
            Special(Some(name)) => format!("special:{}", name),
        }
    }
}

#[derive(Debug)]
pub enum DirectionArgument {
    Left,
    Right,
    Up,
    Down,
}
impl ToString for DirectionArgument {
    fn to_string(&self) -> String {
        use DirectionArgument::*;
        match self {
            Left => "l",
            Right => "r",
            Up => "u",
            Down => "d",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub enum MonitorArgument {
    Direction(DirectionArgument),
    ID(u64),
    Name(String),
    Current,
    Relative(i32),
}
impl ToString for MonitorArgument {
    fn to_string(&self) -> String {
        use MonitorArgument::*;
        match self {
            Direction(dir) => dir.to_string(),
            ID(id) => id.to_string(),
            Name(name) => name.clone(),
            Current => "current".to_string(),
            Relative(rel) => RelAbs::Relative(*rel).to_string(),
        }
    }
}

// TODO: might need to do extra checks: "exact -50 -50" gets us an "ok" response, but is invalid
#[derive(Debug)]
pub enum NumPercent {
    Number(i32),
    Percent(u32),
}
impl fmt::Display for NumPercent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::Percent(percent) => write!(f, "{}%", percent),
        }
    }
}
#[derive(Debug)]
pub enum ResizeArgument {
    Relative(NumPercent, NumPercent),
    Exact(NumPercent, NumPercent),
}
impl ToString for ResizeArgument {
    fn to_string(&self) -> String {
        match self {
            Self::Relative(w, h) => format!("{} {}", w, h),
            Self::Exact(w, h) => format!("exact {} {}", w, h),
        }
    }
}

#[derive(Debug)]
pub enum FloatArgument {
    Relative(f32),
    Exact(f32),
}
impl ToString for FloatArgument {
    fn to_string(&self) -> String {
        match self {
            Self::Relative(float) => float.to_string(),
            Self::Exact(float) => format!("exact {}", float),
        }
    }
}

#[derive(Debug)]
pub enum ZHeightArgument {
    Top,
    Bottom,
}
impl ToString for ZHeightArgument {
    fn to_string(&self) -> String {
        match self {
            Self::Top => "top".to_string(),
            Self::Bottom => "bottom".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ModArgument {
    Shift,
    Caps,
    Ctrl,
    AltMod,
    Mod2,
    Mod3,
    Super,
    Mod5,
}
impl ToString for ModArgument {
    fn to_string(&self) -> String {
        use ModArgument::*;
        match self {
            Shift => "SHIFT",
            Caps => "CAPS",
            Ctrl => "CTRL",
            AltMod => "ALT",
            Mod2 => "MOD2",
            Mod3 => "MOD3",
            Super => "SUPER",
            Mod5 => "MOD5",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub enum KeyArgument {
    Char(char),
    Code(u32),
    Mouse(u32),
}
impl ToString for KeyArgument {
    fn to_string(&self) -> String {
        match self {
            KeyArgument::Char(c) => c.to_string(),
            KeyArgument::Code(code) => format!("code:{}", code),
            KeyArgument::Mouse(code) => format!("mouse:{}", code),
        }
    }
}
