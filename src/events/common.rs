use log::warn;

use crate::events::HyprlandEvent;
use std::collections::HashSet;

use super::HyprlandEventType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParseError {
    MalformedEvent,
    FilteredOut,
}

// TODO: more thorough check
fn parse_bool(arg: &str) -> bool {
    arg.as_bytes()[0] == b'1'
}

fn parse_int(arg: &str) -> i64 {
    arg.parse().unwrap()
}

pub(crate) fn parse_event(msg: &str, filter: &EventFilter) -> Result<HyprlandEvent, ParseError> {
    let ev_name: &str;
    let argv: Vec<&str>;

    if let Some((right, left)) = msg.split_once(">>") {
        ev_name = right;
        argv = left.trim_end().split(',').collect();
    } else {
        return Err(ParseError::MalformedEvent);
    }

    if !filter.includes(ev_name) {
        return Err(ParseError::FilteredOut);
    }

    Ok(match ev_name {
        "workspace" => HyprlandEvent::Workspace {
            name: argv[0].to_owned(),
        },
        "workspacev2" => HyprlandEvent::WorkspaceV2 {
            id: parse_int(argv[0]),
            name: argv[1].to_owned(),
        },
        "focusedmon" => HyprlandEvent::FocusedMonitor {
            name: argv[0].to_owned(),
            workspace_name: argv[1].to_owned(),
        },
        "focusedmonv2" => HyprlandEvent::FocusedMonitorV2 {
            name: argv[0].to_owned(),
            workspace_id: parse_int(argv[1]),
        },
        "activewindow" => HyprlandEvent::ActiveWindow {
            class: argv[0].to_owned(),
            title: argv[1].to_owned(),
        },
        "activewindowv2" => HyprlandEvent::ActiveWindowV2 {
            address: argv[0].to_owned(),
        },
        "fullscreen" => HyprlandEvent::Fullscreen {
            is_fullscreen: parse_bool(argv[0]),
        },
        "monitorremoved" => HyprlandEvent::MonitorRemoved {
            name: argv[0].to_owned(),
        },
        "monitoradded" => HyprlandEvent::MonitorAdded {
            name: argv[0].to_owned(),
        },
        "monitoraddedv2" => HyprlandEvent::MonitorAddedV2 {
            id: parse_int(argv[0]),
            name: argv[1].to_owned(),
            description: argv[2].to_owned(),
        },
        "createworkspace" => HyprlandEvent::CreateWorkspace {
            name: argv[0].to_owned(),
        },
        "createworkspacev2" => HyprlandEvent::CreateWorkspaceV2 {
            id: parse_int(argv[0]),
            name: argv[1].to_owned(),
        },
        "destroyworkspace" => HyprlandEvent::DestroyWorkspace {
            name: argv[0].to_owned(),
        },
        "destroyworkspacev2" => HyprlandEvent::DestroyWorkspaceV2 {
            id: parse_int(argv[0]),
            name: argv[1].to_owned(),
        },
        "moveworkspace" => HyprlandEvent::MoveWorkspace {
            name: argv[0].to_owned(),
            mon_name: argv[1].to_owned(),
        },
        "moveworkspacev2" => HyprlandEvent::MoveWorkspaceV2 {
            id: parse_int(argv[0]),
            name: argv[1].to_owned(),
            mon_name: argv[1].to_owned(),
        },
        "renameworkspace" => HyprlandEvent::RenameWorkspace {
            id: parse_int(argv[0]),
            name: argv[1].to_owned(),
        },
        "activespecial" => HyprlandEvent::ActiveSpecial {
            name: argv[0].to_owned(),
            mon_name: argv[1].to_owned(),
        },
        "activelayout" => HyprlandEvent::ActiveLayout {
            keyboard_name: argv[0].to_owned(),
            layout_name: argv[1].to_owned(),
        },
        "openwindow" => HyprlandEvent::OpenWindow {
            address: argv[0].to_owned(),
            workspace_name: argv[1].to_owned(),
            class: argv[2].to_owned(),
            title: argv[3].to_owned(),
        },
        "closewindow" => HyprlandEvent::CloseWindow {
            address: argv[0].to_owned(),
        },
        "movewindow" => HyprlandEvent::MoveWindow {
            address: argv[0].to_owned(),
            workspace_name: argv[1].to_owned(),
        },
        "movewindowv2" => HyprlandEvent::MoveWindowV2 {
            address: argv[0].to_owned(),
            workspace_id: parse_int(argv[1]),
            workspace_name: argv[2].to_owned(),
        },
        "openlayer" => HyprlandEvent::OpenLayer {
            namespace: argv[0].to_owned(),
        },
        "closelayer" => HyprlandEvent::CloseLayer {
            namespace: argv[0].to_owned(),
        },
        "submap" => HyprlandEvent::Submap {
            name: argv[0].to_owned(),
        },
        "changefloatingmode" => HyprlandEvent::ChangeFloatingMode {
            window_address: argv[0].to_owned(),
            floating: parse_bool(argv[1]),
        },
        "urgent" => HyprlandEvent::Urgent {
            window_address: argv[0].to_owned(),
        },
        "screencast" => HyprlandEvent::Screencast {
            state: parse_bool(argv[0]),
            owner: argv[1].to_owned(),
        },
        "windowtitle" => HyprlandEvent::WindowTitle {
            address: argv[0].to_owned(),
        },
        "windowtitlev2" => HyprlandEvent::WindowTitleV2 {
            address: argv[0].to_owned(),
            title: argv[1].to_owned(),
        },
        "togglegroup" => HyprlandEvent::ToggleGroup {
            state: parse_bool(argv[0]),
            handles: argv.iter().skip(1).map(|&slice| slice.to_owned()).collect(),
        },
        "moveintogroup" => HyprlandEvent::MoveIntoGroup {
            address: argv[0].to_owned(),
        },
        "moveoutofgroup" => HyprlandEvent::MoveOutOfGroup {
            address: argv[0].to_owned(),
        },
        "ignoregrouplock" => HyprlandEvent::IgnoreGroupLock {
            is_on: parse_bool(argv[0]),
        },
        "lockgroups" => HyprlandEvent::LockGroups {
            is_on: parse_bool(argv[0]),
        },
        "configreloaded" => HyprlandEvent::ConfigReloaded,
        "pin" => HyprlandEvent::Pin {
            address: argv[0].to_owned(),
            pin_state: parse_bool(argv[1]),
        },
        _ => {
            if ev_name != "custom" {
                warn!(msg; "Unknown event");
            }
            HyprlandEvent::Custom {
                data: argv.join(","),
            }
        }
    })
}

/// Represents a filter for HyprlandEvents.
/// When the library tries to parse an event, this struct gets passed to the parse function. If the
/// current event doesn't pass the filter, it will not get parsed thus saving both time and memory.
#[derive(Debug, Clone)]
pub struct EventFilter {
    filter_set: HashSet<&'static str>,
    include: bool,
}

impl EventFilter {
    /// Creates a new filter with the specified inclusion.
    pub fn new(include: bool) -> Self {
        EventFilter {
            filter_set: HashSet::new(),
            include,
        }
    }

    /// Creates a new filter that lets all events pass. Equivalent to `EventFilter::new(false)`
    pub fn new_include_all() -> Self {
        Self::new(false)
    }

    /// Creates a new filter that doesn't let anything pass. Equivalent to `EventFilter::new(true)`
    pub fn new_exclude_all() -> Self {
        Self::new(true)
    }

    /// Sets whether events present in the filter should be filtered in or out.
    pub fn set_include(&mut self, include: bool) {
        self.include = include;
    }

    /// Adds an event to the filter.
    pub fn add_event(&mut self, ev_type: &HyprlandEventType) {
        self.filter_set.insert(ev_type.get_name());
    }

    /// Returns whether the specified event passes the filter or not.
    pub fn includes(&self, ev_name: &str) -> bool {
        !(self.filter_set.contains(ev_name) ^ self.include)
    }

    /// Returns whether this filter let's anything pass or it rejects all events.
    pub fn filters_everything(&self) -> bool {
        self.filter_set.is_empty() && self.include
    }
}

impl Default for EventFilter {
    /// Equivalent to `Self::new_exclude_all()`
    fn default() -> Self {
        Self::new_exclude_all()
    }
}

impl<I> FromIterator<I> for EventFilter
where
    I: AsRef<HyprlandEventType>,
{
    /// Returns an EventFilter that lets all events in the iterator pass.
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut filter = Self::new_exclude_all();
        for ev_type in iter {
            filter.add_event(ev_type.as_ref());
        }
        filter
    }
}
