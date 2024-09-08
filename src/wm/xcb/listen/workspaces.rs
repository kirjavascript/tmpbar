use xcb::x;
use xcb_wm::{ewmh, icccm};
use crate::wm::xcb::Event;
use crate::wm::monitor;
use std::collections::HashMap;

pub struct Workspaces {
    current: u32,
    desktops: Vec<(String, u32, u32)>,
    urgency: Vec<u32>,
    monitors: Vec<monitor::Monitor>,
    monitor_current: HashMap<u32, u32>,
}

#[derive(Debug)]
pub struct Workspace {
    pub number: u32,
    pub name: String,
    pub focused: bool,
    pub urgent: bool,
    pub visible: bool,
    pub monitor_index: u32,
}

#[derive(Clone)]
pub enum WorkspaceDirection {
    Next,
    Prev,
}

impl Workspaces {
    pub fn new() -> Self {
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();
        let ewmh_conn = ewmh::Connection::connect(&conn);

        let names = get_desktop_names(&ewmh_conn);
        let origins = get_desktop_origins(&conn, &ewmh_conn, &root);

        let mut workspaces = Workspaces {
            current: get_current_desktop(&ewmh_conn),
            desktops: zip_names_origins(names, origins),
            urgency: get_desktop_urgency(&conn, &ewmh_conn),
            monitors: monitor::list(),
            monitor_current: HashMap::new(),
        };

        let monitor_index = workspaces.get_monitor_index(&workspaces.desktops[workspaces.current as usize]);

        workspaces.monitor_current.insert(monitor_index, workspaces.current);

        workspaces
    }

    pub fn set_current(&mut self, current: u32) {
        self.current = current;
        self.update_last_visible();
    }

    pub fn set_desktops(&mut self, desktops: Vec<(String, u32, u32)>) {
        self.desktops = desktops;
        self.update_last_visible();
    }

    pub fn set_urgency(&mut self, urgency: Vec<u32>) {
        self.urgency = urgency;
    }

    pub fn update_last_visible(&mut self) {
        if self.desktops.len() > self.current as _ {
            let index = self.get_monitor_index(&self.desktops[self.current as usize]);
            self.monitor_current.insert(index, self.current);
        }
    }

    pub fn list(&self) -> Vec<Workspace> {
        self.desktops.iter().enumerate().map(|(i, desktop)| {
            let focused = self.current == i as _;
            let monitor_index = self.get_monitor_index(&desktop);
            let monitor_last = self.monitor_current.get(&monitor_index);
            let visible = if let Some(mcurrent) = monitor_last {
                *mcurrent == i as u32
            } else {
                focused
            };

            Workspace {
                number: i as u32 + 1,
                name: desktop.0.to_owned(),
                focused,
                visible,
                urgent: self.urgency.contains(&(i as u32)),
                monitor_index,
            }
        }).collect()
    }

    pub fn get_monitor_index(&self, desktop: &(String, u32, u32)) -> u32 {
        self.monitors.iter().find(|m| {
            m.x == desktop.1 as _ && m.y == desktop.2 as _
        }).map(|m| m.index).unwrap_or(0)
    }

    pub fn cycle_workspace(&self, direction: WorkspaceDirection) {
        let workspaces = self.list();

        let current = &workspaces[self.current as usize];

        let monitor_index = current.monitor_index;
        let current_number = current.number;

        let workspaces: Vec<Workspace> = workspaces
            .into_iter()
            .filter(|workspace| workspace.monitor_index == monitor_index)
            .collect();

        let current_index = workspaces.iter().position(|workspace| workspace.number == current_number).unwrap_or(0);


        match direction {
            WorkspaceDirection::Next => {
                if current_index + 1 < workspaces.len() {
                    let workspace = &workspaces[current_index + 1].number;
                    self.focus_workspace(workspace - 1);
                }
            },
            WorkspaceDirection::Prev => {
                if current_index > 0 {
                    let workspace = &workspaces[current_index - 1].number;
                    self.focus_workspace(workspace - 1);
                }
            },
        }
    }

    pub fn focus_workspace(&self, workspace: u32) {

        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();
        let ewmh_conn = ewmh::Connection::connect(&conn);

    // let request = ewmh::proto::SendCurrentDesktop::new(&ewmh_conn, workspace);
    // let cookie = ewmh_conn.send_request(&request);

    // ewmh_conn.wait_for_reply(cookie).unwrap();


        println!("{workspace}");
    }
}

pub fn handle_event(
    event: &x::PropertyNotifyEvent,
    root: &x::Window,
    conn: &xcb::Connection,
    ewmh_conn: &ewmh::Connection,
) -> Option<Event> {
    let atom = event.atom();

    if atom == ewmh_conn.atoms._NET_CURRENT_DESKTOP {
        return Some(Event::WorkspaceCurrent(get_current_desktop(&ewmh_conn)));
    }

    if atom == ewmh_conn.atoms._NET_DESKTOP_NAMES {
        let names = get_desktop_names(&ewmh_conn);
        let origins = get_desktop_origins(&conn, &ewmh_conn, &root);

        return Some(Event::WorkspaceDesktops(zip_names_origins(names, origins)));
    }

    if atom == x::ATOM_WM_HINTS {
        return Some(Event::WorkspaceUrgency(get_desktop_urgency(conn, ewmh_conn)));
    }

    return None
}

pub fn get_current_desktop(ewmh_conn: &ewmh::Connection) -> u32 {
    let request = ewmh::proto::GetCurrentDesktop;
    let cookie = ewmh_conn.send_request(&request);
    ewmh_conn.wait_for_reply(cookie).unwrap().desktop
}

pub fn get_desktop_names(ewmh_conn: &ewmh::Connection) -> Vec<String> {
    let request = ewmh::proto::GetDesktopNames;
    let cookie = ewmh_conn.send_request(&request);
    ewmh_conn.wait_for_reply(cookie).unwrap().names
}

pub fn get_desktop_origins(conn: &xcb::Connection, ewmh_conn: &ewmh::Connection, window: &x::Window) -> Vec<(u32, u32)> {
    // bug in xcb-wm: GetDesktopViewport assumes only a single viewport is present
    let cookie = conn.send_request(&xcb::x::GetProperty {
        delete: false,
        window: *window,
        property: ewmh_conn.atoms._NET_DESKTOP_VIEWPORT,
        r#type: xcb::x::ATOM_CARDINAL,
        long_offset: 0,
        long_length: 256,
    });

    let reply = conn.wait_for_reply(cookie).unwrap();
    let value: &[u32] = reply.value();

    value.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect()
}

fn zip_names_origins(names: Vec<String>, origins: Vec<(u32, u32)>) -> Vec<(String, u32, u32)> {
    names.into_iter()
        .zip(origins.into_iter())
        .map(|(s, (a, b))| (s, a, b))
        .collect()
}

pub fn get_client_list(ewmh_conn: &ewmh::Connection) -> Vec<x::Window> {
    let request = ewmh::proto::GetClientList;
    let cookie = ewmh_conn.send_request(&request);
    ewmh_conn.wait_for_reply(cookie).unwrap().clients
}

pub fn get_desktop_urgency(conn: &xcb::Connection, ewmh_conn: &ewmh::Connection) -> Vec<u32> {
    let mut urgent_desktops = std::collections::HashSet::new();

    for window in get_client_list(&ewmh_conn) {
        if let Ok(reply) = conn.wait_for_reply(conn.send_request(&x::GetProperty {
            delete: false,
            window,
            property: xcb::x::ATOM_WM_HINTS,
            r#type: xcb::x::ATOM_WM_HINTS,
            long_offset: 0,
            long_length: 256,
        })) {
            let value = reply.value::<u32>();

            if value.len() > 0 {
                if icccm::proto::WmHints::from_reply(reply).is_urgent() {
                    let request = ewmh::proto::GetWmDesktop(window);
                    let cookie = ewmh_conn.send_request(&request);
                    let desktop = ewmh_conn.wait_for_reply(cookie).unwrap().desktop;

                    urgent_desktops.insert(desktop);
                }
            }
        }
    }

    urgent_desktops.into_iter().collect()
}
