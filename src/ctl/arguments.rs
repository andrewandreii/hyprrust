use core::fmt;

use super::data::{Monitor, Window, Workspace, WorkspaceBrief};

pub trait Argument {
    fn to_argument_string(&self) -> String;
}

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

impl Argument for WindowArgument {
    fn to_argument_string(&self) -> String {
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

impl From<Window> for WindowArgument {
    fn from(value: Window) -> Self {
        WindowArgument::Address(value.address)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RelAbs {
    Relative(i32),
    Absolute(u32),
}

impl Argument for RelAbs {
    fn to_argument_string(&self) -> String {
        match self {
            RelAbs::Relative(id) if *id >= 0 => format!("+{}", id),
            RelAbs::Relative(id) => id.to_string(),
            RelAbs::Absolute(id) => format!("~{}", id),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WorkspaceArgument {
    ID(i64),
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

impl Argument for WorkspaceArgument {
    fn to_argument_string(&self) -> String {
        use WorkspaceArgument::*;
        match self {
            ID(id) => id.to_string(),
            RelativeID(id) => RelAbs::Relative(*id).to_argument_string(),
            WorkspaceOnMonitor(relabs) => format!("m{}", relabs.to_argument_string()),
            WorkspaceOnMonitorWithEmpty(relabs) => format!("r{}", relabs.to_argument_string()),
            OpenWorkspace(relabs) => format!("e{}", relabs.to_argument_string()),
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

impl From<Workspace> for WorkspaceArgument {
    fn from(value: Workspace) -> Self {
        WorkspaceArgument::ID(value.id)
    }
}

impl From<WorkspaceBrief> for WorkspaceArgument {
    fn from(value: WorkspaceBrief) -> Self {
        WorkspaceArgument::ID(value.id)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DirectionArgument {
    Left,
    Right,
    Up,
    Down,
}

impl Argument for DirectionArgument {
    fn to_argument_string(&self) -> String {
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
    ID(i64),
    Name(String),
    Current,
    Relative(i32),
}

impl Argument for MonitorArgument {
    fn to_argument_string(&self) -> String {
        use MonitorArgument::*;
        match self {
            Direction(dir) => dir.to_argument_string(),
            ID(id) => id.to_string(),
            Name(name) => name.clone(),
            Current => "current".to_string(),
            Relative(rel) => RelAbs::Relative(*rel).to_argument_string(),
        }
    }
}

impl From<Monitor> for MonitorArgument {
    fn from(value: Monitor) -> Self {
        MonitorArgument::ID(value.id)
    }
}

// TODO: might need to do extra checks: "exact -50 -50" gets us an "ok" response, but is invalid
#[derive(Debug, Clone, Copy)]
pub enum NumPercent {
    Number(i32),
    Percent(i32),
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

impl Argument for ResizeArgument {
    fn to_argument_string(&self) -> String {
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

impl Argument for FloatArgument {
    fn to_argument_string(&self) -> String {
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

impl Argument for ZHeightArgument {
    fn to_argument_string(&self) -> String {
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

impl Argument for ModArgument {
    fn to_argument_string(&self) -> String {
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

impl Argument for KeyArgument {
    fn to_argument_string(&self) -> String {
        match self {
            KeyArgument::Char(c) => c.to_string(),
            KeyArgument::Code(code) => format!("code:{}", code),
            KeyArgument::Mouse(code) => format!("mouse:{}", code),
        }
    }
}

impl Argument for bool {
    fn to_argument_string(&self) -> String {
        match self {
            true => "1",
            false => "0",
        }
        .to_string()
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

impl Argument for CycleNextArguments {
    fn to_argument_string(&self) -> String {
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

impl Argument for TagArgument {
    fn to_argument_string(&self) -> String {
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

impl Argument for CornerArgument {
    fn to_argument_string(&self) -> String {
        (*self as i32).to_string()
    }
}

impl Argument for i32 {
    fn to_argument_string(&self) -> String {
        self.to_string()
    }
}

impl Argument for String {
    fn to_argument_string(&self) -> String {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ColorArgument {
    hex_str: String,
}

impl ColorArgument {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        ColorArgument {
            hex_str: format!("rgba({:02x}{:02x}{:02x}{:02x})", r, g, b, a),
        }
    }
}

impl Argument for ColorArgument {
    fn to_argument_string(&self) -> String {
        self.hex_str.clone()
    }
}

#[derive(Debug, Clone)]
pub enum NotifyIconArgument {
    NoIcon,
    Warning,
    Info,
    Hint,
    Error,
    Confused,
    Ok,
}

impl Argument for NotifyIconArgument {
    fn to_argument_string(&self) -> String {
        match self {
            Self::NoIcon => "-1",
            Self::Warning => "0",
            Self::Info => "1",
            Self::Hint => "2",
            Self::Error => "3",
            Self::Confused => "4",
            Self::Ok => "5",
        }
        .to_string()
    }
}

pub enum MonitorsDataArgument {
    All,
    JustActive,
}

impl Argument for MonitorsDataArgument {
    fn to_argument_string(&self) -> String {
        match self {
            Self::All => "all".to_string(),
            Self::JustActive => String::new(),
        }
    }
}

pub enum ChangeGroupActiveArgument {
    Index(i32),
    Back,
    Forward,
}

impl Argument for ChangeGroupActiveArgument {
    fn to_argument_string(&self) -> String {
        match self {
            Self::Index(i) => i.to_string(),
            Self::Back => "b".to_string(),
            Self::Forward => "f".to_string(),
        }
    }
}

pub enum LockGroupArgument {
    Lock,
    Unlock,
    Toggle,
}

impl Argument for LockGroupArgument {
    fn to_argument_string(&self) -> String {
        match self {
            LockGroupArgument::Lock => "lock".to_string(),
            LockGroupArgument::Unlock => "unlock".to_string(),
            LockGroupArgument::Toggle => "toggle".to_string(),
        }
    }
}

pub enum BoolChangeArgument {
    On,
    Off,
    Toggle,
}

impl Argument for BoolChangeArgument {
    fn to_argument_string(&self) -> String {
        match self {
            BoolChangeArgument::On => "on".to_string(),
            BoolChangeArgument::Off => "off".to_string(),
            BoolChangeArgument::Toggle => "toggle".to_string(),
        }
    }
}
