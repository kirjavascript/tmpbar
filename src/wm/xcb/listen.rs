use xcb::{x, Xid};

use crate::global::Event;
use crate::util::Signal;

pub fn listen(signal: Signal<Event>) {
    std::thread::spawn(move || {
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();
        let atoms = super::Atoms::intern_all(&conn).unwrap();

        conn.send_request(&x::ChangeWindowAttributes {
            window: root,
            value_list: &[
                x::Cw::EventMask(x::EventMask::PROPERTY_CHANGE),
            ],
        });

        conn.flush().unwrap();

        let mut current_window = x::Window::none();

        loop {
            match conn.wait_for_event() {
                Ok(event) => {
                    match event {
                        xcb::Event::X(x::Event::PropertyNotify(event)) => {
                            let atom = event.atom();



                            let is_active = atom == atoms.active_window;
                            let is_title = is_active || atom == x::ATOM_WM_NAME;

                            if !is_title {
                                continue;
                            }

                            let cookie = conn.send_request(&xcb::x::GetProperty {
                                delete: false,
                                window: root,
                                property: atoms.active_window,
                                r#type: xcb::x::ATOM_WINDOW,
                                long_offset: 0,
                                long_length: 1,
                            });

                            if let Ok(reply) = conn.wait_for_reply(cookie) {

                                let active_windows: &[x::Window] = reply.value();
                                if active_windows.len() != 0 {
                                    let active_window = active_windows[0];

                                    if is_active {
                                        if current_window != active_window {
                                            // unsubscribe old window
                                            if !current_window.is_none() {
                                                conn.send_request(&x::ChangeWindowAttributes {
                                                    window: current_window,
                                                    value_list: &[
                                                        x::Cw::EventMask(x::EventMask::NO_EVENT),
                                                    ],
                                                });
                                            }
                                            // subscribe to new one
                                            if !active_window.is_none() {
                                                conn.send_request(&x::ChangeWindowAttributes {
                                                    window: active_window,
                                                    value_list: &[
                                                        x::Cw::EventMask(x::EventMask::PROPERTY_CHANGE),
                                                    ],
                                                });
                                            }
                                            current_window = active_window;

                                            conn.flush().unwrap();

                                        }

                                    }

                                    let cookie = conn.send_request(&xcb::x::GetProperty {
                                        delete: false,
                                        window: active_window,
                                        property: xcb::x::ATOM_WM_NAME,
                                        r#type: atoms.utf8_string,
                                        long_offset: 0,
                                        long_length: 1024,
                                    });

                                    if let Ok(reply) = conn.wait_for_reply(cookie) {
                                        let value = reply.value();
                                        let title = String::from_utf8_lossy(value).into_owned();

                                        signal.send(Event::WindowTitle(title));
                                    }
                                }
                            }
                        }
                        _ => { },
                    };
                }
                Err(error) => {
                    error!("{}", error);
                },
            }
        }
    });
}

fn handle_title(atom: x::atoms)
