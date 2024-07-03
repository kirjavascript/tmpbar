use xcb::x;
use xcb_wm::{ewmh, icccm};
use crate::global::Event;
use crate::wm::monitor;

// TODO: visible by using tracking
// make global global
// use _monitor_index

pub struct Workspaces {
    pub current: u32,
    pub desktops: Vec<(String, u32, u32)>,
    pub urgency: Vec<u32>,
    monitors: Vec<monitor::Monitor>,
}

#[derive(Debug)]
pub struct Workspace {
    number: u32,
    name: String,
    focused: bool,
    urgent: bool,
    // visible
    monitor: u32,
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

        Workspaces {
            current: get_current_desktop(&ewmh_conn),
            desktops: zip_names_origins(names, origins),
            urgency: get_desktop_urgency(&conn, &ewmh_conn),
            monitors: monitor::list(),
        }
    }

    pub fn list(&self) -> Vec<Workspace> {
        self.desktops.iter().enumerate().map(|(i, desktop)| {
            let monitor = self.monitors.iter().find(|m| {
                m.x == desktop.1 as _ && m.y == desktop.2 as _
            }).map(|m| m.index).unwrap_or(0);

            Workspace {
                number: i as u32 + 1,
                name: desktop.0.to_owned(),
                focused: self.current == i as _,
                urgent: self.urgency.contains(&(i as u32)),
                monitor,
            }
        }).collect()
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
        return Some(Event::WorkspaceCurrent( get_current_desktop(&ewmh_conn)));
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
