use xcb::x;
use xcb_wm::{ewmh, icccm};

pub struct Workspaces {
    current: u32,
    desktops: Vec<(String, u32, u32)>,
    urgency: Vec<u32>,
}

// pub struct Workspace {
// }

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
            urgency: vec![],
        }
    }
}

pub fn handle_event(
    event: &x::PropertyNotifyEvent,
    window: &x::Window,
    conn: &xcb::Connection,
    ewmh_conn: &ewmh::Connection,
    icccm_conn: &icccm::Connection,
) {
    let atom = event.atom();

    if atom == ewmh_conn.atoms._NET_NUMBER_OF_DESKTOPS {
        let request = ewmh::proto::GetNumberOfDesktops;
        let cookie = ewmh_conn.send_request(&request);
        let reply = ewmh_conn.wait_for_reply(cookie).unwrap();
        println!("NO OF {:?}", reply);
    }

    if atom == ewmh_conn.atoms._NET_CURRENT_DESKTOP {
        let request = ewmh::proto::GetCurrentDesktop;
        let cookie = ewmh_conn.send_request(&request);
        let reply = ewmh_conn.wait_for_reply(cookie).unwrap();

        println!("CURRENT_DESKTOP {:?}", reply);
    }

    if atom == ewmh_conn.atoms._NET_DESKTOP_NAMES {
        let request = ewmh::proto::GetDesktopNames;
        let cookie = ewmh_conn.send_request(&request);
        let reply = ewmh_conn.wait_for_reply(cookie).unwrap();

        print!("DESKTOP_NAMES {:?} ", reply);

        let cookie = conn.send_request(&xcb::x::GetProperty {
            delete: false,
            window: *window,
            property: ewmh_conn.atoms._NET_DESKTOP_VIEWPORT,
            r#type: xcb::x::ATOM_CARDINAL,
            long_offset: 0,
            long_length: 1024,
        });

        let reply = conn.wait_for_reply(cookie).unwrap();
        let value: &[u32] = reply.value();

        println!("{:?}", value);

    }

    if atom == x::ATOM_WM_HINTS {
        // https://github.com/polybar/polybar/blob/11b522c313f7b2b0a10063721ec8b0bf544de6f4/src/modules/xworkspaces.cpp#L105
        //https://github.com/polybar/polybar/blob/11b522c313f7b2b0a10063721ec8b0bf544de6f4/src/x11/ewmh.cpp#L135
        let request = icccm::proto::GetWmHints::new(event.window());
        let cookie = icccm_conn.send_request(&request);
        let mut reply = icccm_conn.wait_for_reply(cookie).unwrap();

        // track windows for desktops
        println!("URGENT {:?}", reply.size_hints.is_urgent());
    }

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

pub fn get_desktop_urgency(icccm_conn: &icccm::Connection, ewmh_conn: &ewmh::Connection) -> Vec<u32> {
    let mut urgent_desktops = std::collections::HashSet::new();

    for window in get_client_list(&ewmh_conn) {
        let request = icccm::proto::GetWmHints::new(window);
        let cookie = icccm_conn.send_request(&request);
        let mut reply = icccm_conn.wait_for_reply(cookie).unwrap();

        if reply.size_hints.is_urgent() {
            let request = ewmh::proto::GetWmDesktop(window);
            let cookie = ewmh_conn.send_request(&request);
            let desktop = ewmh_conn.wait_for_reply(cookie).unwrap().desktop;

            urgent_desktops.insert(desktop);
        }
    }

    urgent_desktops.into_iter().collect()
}
