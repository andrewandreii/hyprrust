/// Represents a Hyprland event. The events have the meaning specified here: <https://wiki.hyprland.org/IPC>
#[derive(Clone, Debug)]
pub enum HyprlandEvent {
    Workspace {
        name: String,
    },
    WorkspaceV2 {
        id: i64,
        name: String,
    },
    FocusedMonitor {
        name: String,
        workspace_name: String,
    },
    FocusedMonitorV2 {
        name: String,
        workspace_id: i64,
    },
    ActiveWindow {
        class: String,
        title: String,
    },
    ActiveWindowV2 {
        address: String,
    },
    Fullscreen {
        is_fullscreen: bool,
    },
    MonitorRemoved {
        name: String,
    },
    MonitorAdded {
        name: String,
    },
    MonitorAddedV2 {
        id: i64,
        name: String,
        description: String,
    },
    CreateWorkspace {
        name: String,
    },
    CreateWorkspaceV2 {
        id: i64,
        name: String,
    },
    DestroyWorkspace {
        name: String,
    },
    DestroyWorkspaceV2 {
        id: i64,
        name: String,
    },
    MoveWorkspace {
        name: String,
        mon_name: String,
    },
    MoveWorkspaceV2 {
        id: i64,
        name: String,
        mon_name: String,
    },
    RenameWorkspace {
        id: i64,
        name: String,
    },
    ActiveSpecial {
        name: String,
        mon_name: String,
    },
    ActiveLayout {
        keyboard_name: String,
        layout_name: String,
    },
    OpenWindow {
        address: String,
        workspace_name: String,
        class: String,
        title: String,
    },
    CloseWindow {
        address: String,
    },
    MoveWindow {
        address: String,
        workspace_name: String,
    },
    MoveWindowV2 {
        address: String,
        workspace_id: i64,
        workspace_name: String,
    },
    OpenLayer {
        namespace: String,
    },
    CloseLayer {
        namespace: String,
    },
    Submap {
        name: String,
    },
    ChangeFloatingMode {
        window_address: String,
        floating: bool,
    },
    Urgent {
        window_address: String,
    },
    // TODO: test if struct is correct
    Screencast {
        state: bool,
        owner: String,
    },
    WindowTitle {
        address: String,
    },
    WindowTitleV2 {
        address: String,
        title: String,
    },
    ToggleGroup {
        state: bool,
        handles: Vec<String>,
    },
    MoveIntoGroup {
        address: String,
    },
    MoveOutOfGroup {
        address: String,
    },
    IgnoreGroupLock {
        is_on: bool,
    },
    LockGroups {
        is_on: bool,
    },
    ConfigReloaded,
    Pin {
        address: String,
        pin_state: bool,
    },
    Custome {
        data: String,
    },
}

pub const ALL_EVENTS: &[&'static str] = &[
    "workspace",
    "workspacev2",
    "focusedmon",
    "focusedmonv2",
    "activewindow",
    "activewindowv2",
    "fullscreen",
    "monitorremoved",
    "monitoradded",
    "monitoraddedv2",
    "createworkspace",
    "createworkspacev2",
    "destroyworkspace",
    "destroyworkspacev2",
    "moveworkspace",
    "moveworkspacev2",
    "renameworkspace",
    "activespecial",
    "activelayout",
    "openwindow",
    "closewindow",
    "movewindow",
    "movewindowv2",
    "openlayer",
    "closelayer",
    "submap",
    "changefloatingmode",
    "urgent",
    "screencast",
    "windowtitle",
    "windowtitlev2",
    "togglegroup",
    "moveintogroup",
    "moveoutofgroup",
    "ignoregrouplock",
    "lockgroups",
    "configreloaded",
    "pin",
];
