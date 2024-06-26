use crate::util::Signal;

use std::cell::RefCell;
use std::rc::Rc;

use xcb::x;

pub struct WMUtil(Rc<RefCell<WMUtilInner>>);

struct WMUtilInner {
    window_title: String,
    ctx: egui::Context,
}

impl WMUtil {
    pub fn new(ctx: egui::Context) -> Self {

        // TODO use Global instread of rcrefcell


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
                            xcb::Event::X(x::Event::PropertyNotify(event)) => {

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
                                        println!("{:?}", crate::wm::xcb::get_wm_name(&conn, &window[0]));
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

        WMUtil(Rc::new(RefCell::new(WMUtilInner {
            window_title: "".to_string(),
            ctx,
        })))
    }
}
