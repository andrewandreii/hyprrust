use core::panic;
use std::collections::HashSet;
use std::io;
use std::str;

use crate::connection::EventConnection;
use crate::connection::HyprlandConnection;
use tokio::io::Interest;
use tokio::net::UnixStream;
use tokio::sync::broadcast;
use tokio::time;

pub use super::HyprlandEvent;

#[macro_export]
macro_rules! event_name {
    (HyprlandEvent::$variant:ident) => {
        stringify!($variant).to_ascii_lowercase()
    };
    ($variant:ident) => {
        stringify!($variant).to_ascii_lowercase()
    };
}
pub use event_name;

#[derive(Debug, Clone)]
pub struct EventFilter {
    filter_set: HashSet<String>,
    include: bool,
}
impl EventFilter {
    pub fn all_events() -> Self {
        EventFilter {
            filter_set: HashSet::new(),
            include: false,
        }
    }

    pub fn new_include_all() -> Self {
        Self::all_events()
    }

    pub fn new(include: bool) -> Self {
        EventFilter {
            filter_set: HashSet::new(),
            include,
        }
    }

    pub fn set_include(&mut self, include: bool) {
        self.include = include;
    }

    pub fn add_event(&mut self, ev_name: String) {
        self.filter_set.insert(ev_name);
    }

    pub fn includes(&self, ev_name: &str) -> bool {
        !(self.filter_set.contains(ev_name) ^ self.include)
    }
}

impl Default for EventFilter {
    fn default() -> Self {
        Self::new(true)
    }
}

impl FromIterator<String> for EventFilter {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut filter = Self::default();
        for ev_name in iter {
            filter.add_event(ev_name);
        }
        filter
    }
}

impl<'iter> FromIterator<&'iter String> for EventFilter {
    fn from_iter<T: IntoIterator<Item = &'iter String>>(iter: T) -> Self {
        let mut filter = Self::default();
        for ev_name in iter {
            filter.add_event(ev_name.clone());
        }
        filter
    }
}

impl<'iter> FromIterator<&'iter str> for EventFilter {
    fn from_iter<T: IntoIterator<Item = &'iter str>>(iter: T) -> Self {
        let mut filter = Self::default();
        for ev_name in iter {
            filter.add_event(ev_name.to_string());
        }
        filter
    }
}

fn parse_bool(arg: &str) -> bool {
    return arg.as_bytes()[0] == b'1';
}

fn parse_int(arg: &str) -> i64 {
    return arg.parse().unwrap();
}

// TODO: make a way to exclude certain events if we didn't subsribe to them
fn parse_event(msg: &str, filter: &EventFilter) -> Result<HyprlandEvent, &'static str> {
    let ev_name: &str;
    let argv: Vec<&str>;

    if let Some((right, left)) = msg.split_once(">>") {
        ev_name = right;
        argv = left.split(',').collect();
    } else {
        return Err("Malformed event");
    }

    if !filter.includes(ev_name) {
        return Err("Filtered out");
    }

    match ev_name {
        "workspace" => Ok(HyprlandEvent::Workspace {
            name: argv[0].to_owned(),
        }),
        "workspacev2" => Ok(HyprlandEvent::WorkspaceV2 {
            id: parse_int(argv[0]),
            name: argv[1].to_owned(),
        }),
        "focusedmon" => Ok(HyprlandEvent::FocusedMonitor {
            name: argv[0].to_owned(),
            workspace_name: argv[1].to_owned(),
        }),
        "focusedmonv2" => Ok(HyprlandEvent::FocusedMonitorV2 {
            name: argv[0].to_owned(),
            workspace_id: parse_int(argv[1]),
        }),
        "activewindow" => Ok(HyprlandEvent::ActiveWindow {
            class: argv[0].to_owned(),
            title: argv[1].to_owned(),
        }),
        "activewindowv2" => Ok(HyprlandEvent::ActiveWindowV2 {
            address: argv[0].to_owned(),
        }),
        "fullscreen" => Ok(HyprlandEvent::Fullscreen {
            is_fullscreen: parse_bool(argv[0]),
        }),
        "monitorremoved" => Ok(HyprlandEvent::MonitorRemoved {
            name: argv[0].to_owned(),
        }),
        "monitoradded" => Ok(HyprlandEvent::MonitorAdded {
            name: argv[0].to_owned(),
        }),
        "monitoraddedv2" => Ok(HyprlandEvent::MonitorAddedV2 {
            id: parse_int(argv[0]),
            name: argv[1].to_owned(),
            description: argv[2].to_owned(),
        }),
        "createworkspace" => Ok(HyprlandEvent::CreateWorkspace {
            name: argv[0].to_owned(),
        }),
        "createworkspacev2" => Ok(HyprlandEvent::CreateWorkspaceV2 {
            id: parse_int(argv[0]),
            name: argv[1].to_owned(),
        }),
        "destroyworkspace" => Ok(HyprlandEvent::DestroyWorkspace {
            name: argv[0].to_owned(),
        }),
        "destroyworkspacev2" => Ok(HyprlandEvent::DestroyWorkspaceV2 {
            id: parse_int(argv[0]),
            name: argv[1].to_owned(),
        }),
        "moveworkspace" => Ok(HyprlandEvent::MoveWorkspace {
            name: argv[0].to_owned(),
            mon_name: argv[1].to_owned(),
        }),
        "moveworkspacev2" => Ok(HyprlandEvent::MoveWorkspaceV2 {
            id: parse_int(argv[0]),
            name: argv[1].to_owned(),
            mon_name: argv[1].to_owned(),
        }),
        "renameworkspace" => Ok(HyprlandEvent::RenameWorkspace {
            id: parse_int(argv[0]),
            name: argv[1].to_owned(),
        }),
        "activespecial" => Ok(HyprlandEvent::ActiveSpecial {
            name: argv[0].to_owned(),
            mon_name: argv[1].to_owned(),
        }),
        "activelayout" => Ok(HyprlandEvent::ActiveLayout {
            keyboard_name: argv[0].to_owned(),
            layout_name: argv[1].to_owned(),
        }),
        "openwindow" => Ok(HyprlandEvent::OpenWindow {
            address: argv[0].to_owned(),
            workspace_name: argv[1].to_owned(),
            class: argv[2].to_owned(),
            title: argv[3].to_owned(),
        }),
        "closewindow" => Ok(HyprlandEvent::CloseWindow {
            address: argv[0].to_owned(),
        }),
        "movewindow" => Ok(HyprlandEvent::MoveWindow {
            address: argv[0].to_owned(),
            workspace_name: argv[1].to_owned(),
        }),
        "movewindowv2" => Ok(HyprlandEvent::MoveWindowV2 {
            address: argv[0].to_owned(),
            workspace_id: parse_int(argv[1]),
            workspace_name: argv[2].to_owned(),
        }),
        "openlayer" => Ok(HyprlandEvent::OpenLayer {
            namespace: argv[0].to_owned(),
        }),
        "closelayer" => Ok(HyprlandEvent::CloseLayer {
            namespace: argv[0].to_owned(),
        }),
        "submap" => Ok(HyprlandEvent::Submap {
            name: argv[0].to_owned(),
        }),
        "changefloatingmode" => Ok(HyprlandEvent::ChangeFloatingMode {
            window_address: argv[0].to_owned(),
            floating: parse_bool(argv[1]),
        }),
        "urgent" => Ok(HyprlandEvent::Urgent {
            window_address: argv[0].to_owned(),
        }),
        "screencast" => Ok(HyprlandEvent::Screencast {
            state: parse_bool(argv[0]),
            owner: argv[1].to_owned(),
        }),
        "windowtitle" => Ok(HyprlandEvent::WindowTitle {
            address: argv[0].to_owned(),
        }),
        "windowtitlev2" => Ok(HyprlandEvent::WindowTitleV2 {
            address: argv[0].to_owned(),
            title: argv[1].to_owned(),
        }),
        "togglegroup" => Ok(HyprlandEvent::ToggleGroup {
            state: parse_bool(argv[0]),
            handles: argv.iter().skip(1).map(|&slice| slice.to_owned()).collect(),
        }),
        "moveintogroup" => Ok(HyprlandEvent::MoveIntoGroup {
            address: argv[0].to_owned(),
        }),
        "moveoutofgroup" => Ok(HyprlandEvent::MoveOutOfGroup {
            address: argv[0].to_owned(),
        }),
        "ignoregrouplock" => Ok(HyprlandEvent::IgnoreGroupLock {
            is_on: parse_bool(argv[0]),
        }),
        "lockgroups" => Ok(HyprlandEvent::LockGroups {
            is_on: parse_bool(argv[0]),
        }),
        "configreloaded" => Ok(HyprlandEvent::ConfigReloaded),
        "pin" => Ok(HyprlandEvent::Pin {
            address: argv[0].to_owned(),
            pin_state: parse_bool(argv[1]),
        }),
        _ => Ok(HyprlandEvent::Custome {
            data: argv.join(","),
        }),
    }
}

// TODO: unsure about returning broadcast::Receiver<_>
impl HyprlandConnection {
    /// Spawns a task that listens to Hyprland events and sends them through an async channel. If a
    /// connection already exists, it gets restarted with the new filter provided.
    ///
    /// Once created, the listening task won't die even if all receivers have been dropped. To
    /// kill it call [`stop_listening`].
    ///
    /// [`stop_listening`]: #method.stop_listening
    pub async fn listen_to_events(
        &mut self,
        filter: Option<EventFilter>,
    ) -> Result<broadcast::Receiver<HyprlandEvent>, io::Error> {
        if let Some(_) = self.event_connection.as_ref() {
            self.stop_listening();
        }

        let mut path = self.get_socket_path()?;
        path.push(".socket2.sock");
        let socket = UnixStream::connect(path).await?;

        let (tx, rx) = broadcast::channel(16);

        let filter = filter.unwrap_or_else(|| EventFilter::new_include_all());

        let abort_handle = tokio::spawn(async move {
            'main_loop: loop {
                socket.ready(Interest::READABLE).await.unwrap();

                let mut buf = [0; 1024];
                match socket.try_read(&mut buf) {
                    Ok(len) => {
                        let str_buf = str::from_utf8(&buf[..len])
                            .unwrap()
                            .strip_suffix('\n')
                            .unwrap();
                        for line in str_buf.split('\n') {
                            if let Ok(event) = parse_event(line, &filter) {
                                if let Err(_) = tx.send(event) {
                                    break 'main_loop;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        time::sleep(time::Duration::from_millis(100)).await;
                    }
                }
            }
        })
        .abort_handle();

        let rx_clone = rx.resubscribe();
        self.event_connection = Some(EventConnection {
            abort_handle,
            receiver: rx,
        });

        Ok(rx_clone)
    }

    /// Resubscribes to the event receiver. Must be called after [`listen_to_events`].
    ///
    /// [`listen_to_events`]: #method.listen_to_events
    pub fn resubscribe_to_events(&self) -> broadcast::Receiver<HyprlandEvent> {
        if let Some(ev_conn) = &self.event_connection {
            ev_conn.receiver.resubscribe()
        } else {
            panic!("Listener task hasn't been started");
        }
    }

    /// Returns whether this connection is currently listening to events or not
    pub fn is_listening_to_events(&self) -> bool {
        if let Some(event_connection) = self.event_connection.as_ref() {
            !event_connection.abort_handle.is_finished()
        } else {
            false
        }
    }

    /// Aborts the task that is currently sending events through the broadcast channel. The
    /// broadcast channel also closes. If there is no connection to the event socket, this function
    /// does nothing.
    pub fn stop_listening(&mut self) {
        if let Some(event_connection) = self.event_connection.as_ref() {
            event_connection.abort_handle.abort();
            self.event_connection = None;
        }
    }
}
