use xcb::x;
use crate::global::Event;
use crate::util::Signal;

pub fn watch_window_title(signal: Signal<Event>) {
    std::thread::spawn(move || {
        xcb::atoms_struct! {
            #[derive(Copy, Clone, Debug)]
            pub(crate) struct Atoms {
                pub active_window => b"_NET_ACTIVE_WINDOW",
            }
        }

        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();
        let atoms = Atoms::intern_all(&conn).unwrap();

        conn.send_request(&x::ChangeWindowAttributes {
            window: root,
            value_list: &[
                x::Cw::EventMask(x::EventMask::PROPERTY_CHANGE),
            ],
        });

        conn.flush().unwrap();

        loop {
            match conn.wait_for_event() {
                Ok(event) => {
                    match event {
                        xcb::Event::X(x::Event::PropertyNotify(_)) => {
                            let cookie = conn.send_request(&xcb::x::GetProperty {
                                delete: false,
                                window: root,
                                property: atoms.active_window,
                                r#type: xcb::x::ATOM_WINDOW,
                                long_offset: 0,
                                long_length: 1,
                            });

                            if let Ok(reply) = conn.wait_for_reply(cookie) {

                                let window: &[x::Window] = reply.value();
                                if window.len() != 0 {
                                    if let Some(title) = crate::wm::xcb::get_wm_name(&conn, &window[0]) {
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
