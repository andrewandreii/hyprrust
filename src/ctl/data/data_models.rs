use std::collections::HashMap;

use crate::arguments::{Argument, MonitorsDataArgument, WindowArgument};

use super::{HyprlandData, HyprlandDataWithArgument};
use hyprrust_macros::{HyprlandData, HyprlandDataWithArgument};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

macro_rules! auto_deref {
    ($name:ident = $alias:ty) => {
        impl Deref for $name {
            type Target = $alias;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

#[derive(Serialize, Debug)]
pub struct Sides {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Serialize, Deserialize, Debug, HyprlandData)]
pub struct Version {
    pub branch: String,
    pub commit: String,
    pub version: String,
    pub dirty: bool,
    pub commit_message: String,
    pub commit_date: String,
    pub tag: String,
    pub commits: String,
    #[serde(rename = "buildAquamarine")]
    pub build_aquamarine: String,
    #[serde(rename = "buildHyprlang")]
    pub build_hyprlang: String,
    #[serde(rename = "buildHyprutils")]
    pub build_hyprutils: String,
    #[serde(rename = "buildHyprcursor")]
    pub build_hyprcursor: String,
    #[serde(rename = "buildHyprgraphics")]
    pub build_hyprgraphics: String,
    pub flags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceBrief {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub width: i32,
    pub height: i32,
    pub refresh_rate: f64,
    pub x: i32,
    pub y: i32,
    pub active_workspace: WorkspaceBrief,
    pub special_workspace: WorkspaceBrief,
    pub reserved: Sides,
    pub scale: f64,
    pub transform: i32,
    pub focused: bool,
    pub dpms_status: bool,
    pub vrr: bool,
    pub solitary: String,
    pub actively_tearing: bool,
    pub direct_scanout_to: String,
    pub disabled: bool,
    pub current_format: String,
    pub mirror_of: String,
    pub available_modes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, HyprlandData, HyprlandDataWithArgument)]
#[command(arg_type = MonitorsDataArgument)]
pub struct Monitors(Vec<Monitor>);
auto_deref!(Monitors = Vec<Monitor>);

#[derive(Serialize, Deserialize, Debug, HyprlandData)]
#[command = "activeworkspace"]
pub struct Workspace {
    pub id: i64,
    pub name: String,
    pub monitor: String,
    pub windows: i32,
    #[serde(rename = "hasfullscreen")]
    pub has_fullscreen: bool,
    #[serde(rename = "lastwindow")]
    pub last_window_address: String,
    #[serde(rename = "lastwindowtitle")]
    pub last_window_title: String,
}
pub type CurrentWorkspace = Workspace;

#[derive(Serialize, Deserialize, Debug, HyprlandData)]
pub struct Workspaces(Vec<Workspace>);
auto_deref!(Workspaces = Vec<Workspace>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceRule {
    pub workspace_string: String,
    pub monitor: Option<String>,
    pub default: Option<bool>,
    pub gaps_in: Option<Sides>,
    pub gaps_out: Option<Sides>,
    pub border_size: Option<i32>,
    pub border: Option<bool>,
    pub shadow: Option<bool>,
    pub rounding: Option<bool>,
    pub decorate: Option<bool>,
    pub persistent: Option<bool>,
    pub on_created_empty: Option<String>,
    pub default_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, HyprlandData)]
pub struct WorkspaceRules(Vec<WorkspaceRule>);
auto_deref!(WorkspaceRules = Vec<WorkspaceRule>);

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum FullscreenState {
    None = 0,
    Maximized = 1,
    Fullscreen = 2,
    Max = 3,
}

#[derive(Serialize, Deserialize, Debug, HyprlandData)]
#[serde(rename_all = "camelCase")]
#[command = "activewindow"]
pub struct Window {
    pub address: String,
    pub mapped: bool,
    pub hidden: bool,
    pub at: [i32; 2],
    pub size: [i32; 2],
    pub workspace: WorkspaceBrief,
    pub floating: bool,
    pub pseudo: bool,
    pub monitor: i64,
    pub class: String,
    pub title: String,
    pub initial_class: String,
    pub initial_title: String,
    pub pid: i32,
    pub xwayland: bool,
    pub pinned: bool,
    pub fullscreen: FullscreenState,
    pub fullscreen_client: FullscreenState,
    #[serde(rename = "grouped")]
    pub grouped_windows: Vec<String>,
    pub tags: Vec<String>,
    pub swallowing: String,
    #[serde(rename = "focusHistoryID")]
    pub focus_history_id: i32,
    pub inhibiting_idle: bool,
}
pub type CurrentWindow = Window;

#[derive(Serialize, Deserialize, Debug, HyprlandData)]
#[command = "clients"]
pub struct Windows(Vec<Window>);
auto_deref!(Windows = Vec<Window>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Mouse {
    pub address: String,
    pub name: String,
    pub default_speed: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Keyboard {
    pub address: String,
    pub name: String,
    pub rules: String,
    pub model: String,
    pub layout: String,
    pub variant: String,
    pub options: String,
    #[serde(rename = "active_keymap")]
    pub active_keymap: String,
    pub caps_lock: bool,
    pub num_lock: bool,
    pub main: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TabletPad {
    pub address: String,
    pub name: String,
    pub belongs_to: Tablet,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tablet {
    pub address: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TabletTool {
    pub address: String,
    #[serde(rename = "type")]
    pub tool_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TabletVariant {
    Pad(TabletPad),
    Tablet(Tablet),
    Tool(TabletTool),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Touch {
    pub address: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Switch {
    pub address: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, HyprlandData)]
pub struct Devices {
    pub mice: Vec<Mouse>,
    pub keyboards: Vec<Keyboard>,
    pub tablets: Vec<TabletVariant>,
    pub touch: Vec<Touch>,
    pub switches: Vec<Switch>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Decoration {
    pub decoration_name: String,
    pub priority: u32,
}

#[derive(Serialize, Deserialize, Debug, HyprlandDataWithArgument)]
#[command(arg_type = WindowArgument)]
pub struct Decorations(Vec<Decoration>);
auto_deref!(Decorations = Vec<Decoration>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Bind {
    pub locked: bool,
    pub mouse: bool,
    pub release: bool,
    pub repeat: bool,
    #[serde(rename = "longPress")]
    pub long_press: bool,
    pub non_consuming: bool,
    pub has_description: bool,
    pub modmask: u32,
    pub submap: String,
    pub key: String,
    pub catch_all: bool,
    pub description: String,
    pub dispatcher: String,
    pub arg: String,
}

#[derive(Serialize, Deserialize, Debug, HyprlandData)]
pub struct Binds(Vec<Bind>);
auto_deref!(Binds = Vec<Bind>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Layer {
    pub address: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub namespace: String,
}

#[derive(Serialize, Debug)]
pub struct Levels(pub HashMap<String, Vec<Layer>>);

#[derive(Serialize, Deserialize, Debug, HyprlandData)]
#[serde(transparent)]
pub struct Layers {
    pub monitors: HashMap<String, Levels>,
}

// NOTE: splash not implemented
#[derive(Serialize, Deserialize, Debug, HyprlandDataWithArgument)]
#[command = "getoption"]
pub struct HyprlandOption {
    #[serde(rename = "option")]
    path: String,
    // TODO: is it better to have an enum for the values?
    #[serde(rename = "int", default)]
    int_value: i32,
    #[serde(rename = "float", default)]
    float_value: f32,
    #[serde(rename = "str", default)]
    str_value: String,
    #[serde(rename = "custom", default)]
    custom_value: String,
    #[serde(rename = "vec2", default)]
    vec2_value: [f32; 2],
    set: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, HyprlandData)]
#[command = "cursorpos"]
pub struct CursorPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    pub name: String,
    pub overridden: bool,
    pub bezier: String,
    pub enabled: bool,
    pub speed: f32,
    pub style: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bezier {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AnimationsOrBeziers {
    Animations(Vec<Animation>),
    Beziers(Vec<Bezier>),
}

#[derive(Serialize, Deserialize, Debug, HyprlandData)]
pub struct Animations(Vec<AnimationsOrBeziers>);
auto_deref!(Animations = Vec<AnimationsOrBeziers>);

#[derive(Serialize, Deserialize, Debug, HyprlandData)]
#[serde(transparent)]
pub struct ConfigErrors {
    errors: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, HyprlandData)]
#[serde(transparent)]
pub struct Layouts {
    layouts: Vec<String>,
}
