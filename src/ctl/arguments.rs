use core::fmt;
use std::ops::Deref;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, Copy)]
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
impl From<i32> for NumPercent {
    fn from(value: i32) -> Self {
        NumPercent::Number(value)
    }
}
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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
impl From<f32> for FloatArgument {
    /// Assumes you want FloatArgument::Exact
    fn from(value: f32) -> Self {
        FloatArgument::Exact(value)
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct BoolArgument(bool);
impl ToString for BoolArgument {
    fn to_string(&self) -> String {
        match self.0 {
            true => "1".to_string(),
            false => "0".to_string(),
        }
    }
}
impl From<bool> for BoolArgument {
    fn from(value: bool) -> Self {
        BoolArgument(value)
    }
}

/// Arguments to supply to the CycleNext command.
///
/// # Panics
///
/// While Hyprland doesn't theoretically error when both "floating" and "tiled" are supplied, we
/// shouldn't allow it as it doesn't make sense.
#[derive(Debug, Clone, Copy)]
pub struct CycleNextArguments {
    use_focus_history: bool,
    visible: bool,
    floating: bool,
    tiled: bool,
}
impl CycleNextArguments {
    pub fn new(tiled: bool, floating: bool, visible: bool, use_focus_history: bool) -> Self {
        CycleNextArguments {
            tiled,
            floating,
            visible,
            use_focus_history,
        }
    }

    pub fn with_all_off() -> Self {
        Self::new(false, false, false, false)
    }
}
fn str_if(cond: bool, s: &str) -> &str {
    if cond {
        s
    } else {
        ""
    }
}
impl Default for CycleNextArguments {
    fn default() -> Self {
        Self::with_all_off()
    }
}
impl ToString for CycleNextArguments {
    fn to_string(&self) -> String {
        format!(
            "{} {} {} {}",
            str_if(self.visible, "visible"),
            str_if(self.floating, "floating"),
            str_if(self.tiled, "tiled"),
            str_if(self.use_focus_history, "hist")
        )
    }
}

#[derive(Debug, Clone)]
pub enum TagArgument {
    Set(String),
    Unset(String),
    Toggle(String),
}
impl ToString for TagArgument {
    fn to_string(&self) -> String {
        match self {
            Self::Set(tag) => format!("+{}", tag),
            Self::Unset(tag) => format!("-{}", tag),
            Self::Toggle(tag) => tag.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CornerArgument {
    BottomLeft = 0,
    BottomRight = 1,
    TopRight = 2,
    TopLeft = 3,
}
impl ToString for CornerArgument {
    fn to_string(&self) -> String {
        (*self as i32).to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IntArgument(i32);
impl ToString for IntArgument {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
impl From<i32> for IntArgument {
    fn from(value: i32) -> Self {
        IntArgument(value)
    }
}
impl Into<i32> for IntArgument {
    fn into(self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct StringArgument(String);
impl ToString for StringArgument {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
impl From<String> for StringArgument {
    fn from(value: String) -> Self {
        StringArgument(value)
    }
}
impl Into<String> for StringArgument {
    fn into(self) -> String {
        self.0
    }
}
