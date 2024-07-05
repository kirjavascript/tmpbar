pub mod window_title;
pub mod workspaces;

use xcb::{x, Xid};
use xcb_wm::ewmh;

use crate::global::Event;
use crate::util::Signal;

xcb::atoms_struct! {
    #[derive(Copy, Clone, Debug)]
    pub(crate) struct Atoms {
        pub active_window => b"_NET_ACTIVE_WINDOW",
        pub utf8_string => b"ATOM_UTF8_STRING",
    }
}

pub fn listen(signal: Signal<Event>) {
    std::thread::spawn(move || {
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();
        let atoms = Atoms::intern_all(&conn).unwrap();

        subscribe_windows(&conn, &root);

        let ewmh_conn = ewmh::Connection::connect(&conn);
        let mut active_window = x::Window::none();

        loop {
            match conn.wait_for_event() {
                Ok(event) => {
                    match event {
                        xcb::Event::X(x::Event::CreateNotify(event)) => {
                            subscribe_events(&conn, &event.window());
                        },
                        xcb::Event::X(x::Event::PropertyNotify(event)) => {
                            if let Some(event) = workspaces::handle_event(
                                &event,
                                &root,
                                &conn,
                                &ewmh_conn,
                            ) {
                                signal.send(event);
                            }

                            if let Some(title) = window_title::handle_event(
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
                Err(_error) => {
                    // #[cfg(debug_assertions)]
                    // error!("{:?}", _error);
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

    if let Ok(tree) = conn.wait_for_reply(cookie) {
        for child in tree.children() {
            subscribe_windows(conn, child);
        }
    }
}
