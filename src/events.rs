use std::io;
use std::str;

use crate::connection::HyprlandConnection;
use tokio::io::Interest;
use tokio::net::UnixStream;
use tokio::sync::broadcast;
use tokio::time;

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

fn parse_bool(arg: &str) -> bool {
    return arg.as_bytes()[0] == b'1';
}

fn parse_int(arg: &str) -> i64 {
    return arg.parse().unwrap();
}

fn parse_event(msg: &str) -> Result<HyprlandEvent, &'static str> {
    let ev_name: &str;
    let argv: Vec<&str>;

    if let Some((right, left)) = msg.split_once(">>") {
        ev_name = right;
        argv = left.split(',').collect();
    } else {
        return Err("Malformed event");
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

impl HyprlandConnection {
    // Spawns a task that listens to Hyprland events and sends them through an async channel
    pub async fn listen_to_events(
        &mut self,
    ) -> Result<broadcast::Receiver<HyprlandEvent>, io::Error> {
        if let Some(handle) = self.event_handle.as_ref() {
            handle.abort()
        }

        let mut path = self.get_socket_path()?;
        path.push(".socket2.sock");
        let socket = UnixStream::connect(path).await?;

        let (tx, rx) = broadcast::channel(16);

        self.event_handle = Some(
            tokio::spawn(async move {
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
                                if let Ok(event) = parse_event(line) {
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
            .abort_handle(),
        );

        Ok(rx)
    }

    /// Returns whether a certain connection is currently listening to events or not
    pub fn is_listening_to_events(&self) -> bool {
        if let Some(handle) = self.event_handle.as_ref() {
            !handle.is_finished()
        } else {
            false
        }
    }

    /// Aborts the task that is currently sending events through the broadcast channel. The
    /// broadcast channel also closes.
    pub fn stop_listening(&self) {
        if let Some(handle) = self.event_handle.as_ref() {
            handle.abort();
        }
    }
}
