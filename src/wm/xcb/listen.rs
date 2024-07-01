use xcb::{x, Xid};
use xcb_wm::{ewmh, icccm};

use crate::global::Event;
use crate::util::Signal;

pub fn listen(signal: Signal<Event>) {
    std::thread::spawn(move || {
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();
        let atoms = super::Atoms::intern_all(&conn).unwrap();

        subscribe_windows(&conn, &root);

        let ewmh_conn = ewmh::Connection::connect(&conn);
        let icccm_conn = icccm::Connection::connect(&conn);
        let mut active_window = x::Window::none();

        loop {
            match conn.wait_for_event() {
                Ok(event) => {
                    match event {
                        xcb::Event::X(x::Event::CreateNotify(event)) => {
                            subscribe_events(&conn, &event.window());
                        },
                        xcb::Event::X(x::Event::PropertyNotify(event)) => {
                            wm_hints(
                                &event,
                                &root,
                                &conn,
                                &ewmh_conn,
                                &icccm_conn,
                            );

                            if let Some(title) = window_title(
                                &event,
                                &root,
                                &conn,
                                &atoms,
                                &mut active_window,
                            ) {
                                signal.send(Event::WindowTitle(title));
                            }
                        }
                        _ => {
                        },
                    };
                }
                Err(error) => {
                    #[cfg(debug_assertions)]
                    error!("{}", error);
                },
            }
        }
    });
}

fn subscribe_events(conn: &xcb::Connection, window: &x::Window) {
    conn.send_request(&x::ChangeWindowAttributes{
        window: *window,
        value_list: &[
            x::Cw::EventMask(x::EventMask::PROPERTY_CHANGE | x::EventMask::STRUCTURE_NOTIFY | x::EventMask::SUBSTRUCTURE_NOTIFY),
        ],
    });
    conn.flush().unwrap();
}

fn subscribe_windows(conn: &xcb::Connection, window: &x::Window) {
    subscribe_events(conn, window);

    let cookie = conn.send_request(&xcb::x::QueryTree {
        window: *window,
    });
    let tree = conn.wait_for_reply(cookie).unwrap();

    for child in tree.children() {
        subscribe_windows(conn, child);
    }
}

fn window_title(
    event: &x::PropertyNotifyEvent,
    root: &x::Window,
    conn: &xcb::Connection,
    atoms: &super::Atoms,
    active_window: &mut x::Window
) -> Option<String> {
    let atom = event.atom();

    let is_active = atom == atoms.active_window;
    let is_name = atom == x::ATOM_WM_NAME && event.window() == *active_window;

    if is_active {
        let cookie = conn.send_request(&xcb::x::GetProperty {
            delete: false,
            window: *root,
            property: atoms.active_window,
            r#type: xcb::x::ATOM_WINDOW,
            long_offset: 0,
            long_length: 1,
        });

        if let Ok(reply) = conn.wait_for_reply(cookie) {

            let active_windows: &[x::Window] = reply.value();
            if active_windows.len() != 0 {
                *active_window = active_windows[0];
            }
        }
    }

    if is_active || is_name {
        let cookie = conn.send_request(&xcb::x::GetProperty {
            delete: false,
            window: *active_window,
            property: xcb::x::ATOM_WM_NAME,
            r#type: atoms.utf8_string,
            long_offset: 0,
            long_length: 1024,
        });

        if let Ok(reply) = conn.wait_for_reply(cookie) {
            let value = reply.value();
            let title = String::from_utf8_lossy(value).into_owned();

            return Some(title)
        }
    }

    None
}

fn wm_hints(
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
        // println!("{:?}", reply);
    }

    if atom == ewmh_conn.atoms._NET_CURRENT_DESKTOP {
        let request = ewmh::proto::GetCurrentDesktop;
        let cookie = ewmh_conn.send_request(&request);
        let reply = ewmh_conn.wait_for_reply(cookie).unwrap();

        // println!("{:?}", reply);
    }

    if atom == ewmh_conn.atoms._NET_DESKTOP_NAMES {
        let request = ewmh::proto::GetDesktopNames;
        let cookie = ewmh_conn.send_request(&request);
        let reply = ewmh_conn.wait_for_reply(cookie).unwrap();

        // println!("{:?}", reply);

        // bug in xcb-wm: GetDesktopViewport assumes only a single viewport is present
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

        // println!("{:?}", value);

    }

    if atom == x::ATOM_WM_HINTS {
        // https://github.com/polybar/polybar/blob/11b522c313f7b2b0a10063721ec8b0bf544de6f4/src/modules/xworkspaces.cpp#L105
        let request = icccm::proto::GetWmHints::new(event.window());
        let cookie = icccm_conn.send_request(&request);
        let mut reply = icccm_conn.wait_for_reply(cookie).unwrap();

        println!("{:?}", reply.size_hints.is_urgent());
    }

}
