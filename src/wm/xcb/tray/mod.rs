use xcb::{x, Xid, XidNew};

xcb::atoms_struct! {
    #[derive(Copy, Clone, Debug)]
    pub(crate) struct Atoms {
        pub net_system_tray_orientation => b"_NET_SYSTEM_TRAY_ORIENTATION",
        pub net_wm_state => b"_NET_WM_STATE",
        pub net_wm_state_skip_taskbar => b"_NET_WM_STATE_SKIP_TASKBAR",
        pub wm_protocols => b"WM_PROTOCOLS",
        pub wm_delete_window => b"WM_DELETE_WINDOW",
        pub wm_take_focus => b"WM_TAKE_FOCUS",
        pub net_wm_state_sticky => b"_NET_WM_STATE_STICKY",
        pub net_wm_state_above => b"_NET_WM_STATE_ABOVE",
        pub net_system_tray_s0 => b"_NET_SYSTEM_TRAY_S0",
        pub net_system_tray_opcode => b"_NET_SYSTEM_TRAY_OPCODE",
        pub manager => b"MANAGER",
        // pub net_wm_window_type => b"_NET_WM_WINDOW_TYPE",
        // pub net_wm_window_type_dock => b"_NET_WM_WINDOW_TYPE_DOCK",
    }
}

pub fn tmp_tray() { std::thread::spawn(_tmp_tray); }
// TODO: finish: cleanup
// TODO: icon sizes
// use signal_hook::{consts::SIGINT, iterator::Signals};
// use std::{error::Error, thread, time::Duration};

// fn main() -> Result<(), Box<dyn Error>> {
//     let mut signals = Signals::new(&[SIGINT])?;

//     thread::spawn(move || {
//         for sig in signals.forever() {
//             println!("Received signal {:?}", sig);
//         }
//     });

//     // Following code does the actual work, and can be interrupted by pressing
//     // Ctrl-C. As an example: Let's wait a few seconds.
//     thread::sleep(Duration::from_secs(2));

//     Ok(())
// }

pub fn _tmp_tray() {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let root = screen.root();
    let atoms = Atoms::intern_all(&conn).unwrap();

    if !is_available(&conn, &atoms) {
        warn!("system tray is already in use!");
        return;
    }

    let tray_window = conn.generate_id();

    setup_window(
        &conn,
        tray_window,
        &screen,
        root,
        atoms,
    );

    let mut booted = false;
    let mut icons = vec![];
    let icon_size = 40;

    loop {
        if let Ok(event) = conn.wait_for_event() {
            match event {
                xcb::Event::X(x::Event::PropertyNotify(event)) => {
                    if !booted {
                        take_ownership(
                            &conn,
                            tray_window,
                            root,
                            atoms,
                            &event,
                        );

                        booted = true;
                    }
                },
                xcb::Event::X(x::Event::ClientMessage(event)) => {
                    if event.r#type() == atoms.net_system_tray_opcode {
                        add_icon(
                            &conn,
                            event,
                            tray_window,
                            icon_size,
                            &mut icons,
                        );
                    }
                },
                xcb::Event::X(x::Event::ReparentNotify(event)) => {
                    if event.parent() != tray_window {
                        remove_icon(
                            &conn,
                            event.window(),
                            icon_size,
                            &mut icons,
                        );
                    }
                },
                xcb::Event::X(x::Event::DestroyNotify(event)) => {
                    remove_icon(
                        &conn,
                        event.window(),
                        icon_size,
                        &mut icons,
                    );
                },
                xcb::Event::X(x::Event::Expose(_)) => {
                    let cookie = conn.send_request(&x::GetImage {
                        format: x::ImageFormat::ZPixmap,
                        drawable: x::Drawable::Window(tray_window),
                        x: 0,
                        y: 0,
                        width: icon_size as _,
                        height: icon_size as _,
                        plane_mask: !0,
                    });
                    let reply = conn.wait_for_reply(cookie).unwrap();

                    for c in reply.data().chunks(icon_size as _) {
                        for c in c.chunks(4) {
                            print!("{:0>2X}{:0>2X}{:0>2X}",c[0],c[1],c[2]);
                        }
                        println!("");
                    }
                    println!("{}", reply.data().len());
                },
                xcb::Event::X(x::Event::ButtonPress(event)) => {
                    // TODO: remote press from event mask

                    let window = icons[0];
                    conn.send_request_checked(&x::SendEvent {
                        propagate: false,
                        destination: x::SendEventDest::Window(window),
                        event_mask: x::EventMask::NO_EVENT,

                        event: &x::ButtonPressEvent::new(
                            event.detail(),
                            x::CURRENT_TIME,
                            root,
                            icons[0],
                            x::Window::none(),
                            0,
                            0,
                            0,
                            0,
                            x::KeyButMask::all(),
                            true,
                        ),
                    });

                    conn.flush().unwrap();
                },
                _ => {
                    println!("event {:?}", event);

                },
            };

        }
    }
}

fn add_icon(
    conn: &xcb::Connection,
    event: x::ClientMessageEvent,
    tray_window: x::Window,
    icon_size: u32,
    icons: &mut Vec<x::Window>,
) {
    if let x::ClientMessageData::Data32(data) = event.data() {
        let opcode = data[1];
        let window = data[2];
        let window = unsafe { x::Window::new(window) };

        if opcode == 0 {
            // SYSTEM_TRAY_REQUEST_DOCK

            conn.send_request(&x::ChangeWindowAttributes{
                window,
                value_list: &[
                    x::Cw::EventMask(x::EventMask::STRUCTURE_NOTIFY | x::EventMask::EXPOSURE),
                ],
            });
            conn.send_request(&x::ReparentWindow{
                window,
                parent: tray_window,
                x: icon_size as i16 * icons.len() as i16,
                y: 0,
            });
            conn.send_request(&x::MapWindow {
                window,
            });

            conn.send_request(&xcb::x::ConfigureWindow {
                window,
                value_list: &[
                    xcb::x::ConfigWindow::Width(icon_size),
                    xcb::x::ConfigWindow::Height(icon_size),
                ],
            });
            conn.flush().unwrap();
            icons.push(window);
        }
    }
}

fn remove_icon(
    conn: &xcb::Connection,
    window: x::Window,
    icon_size: u32,
    icons: &mut Vec<x::Window>,
) {
    icons.retain(|child| *child != window);

    for (index, child) in icons.iter().enumerate() {
        let window = *child;
        let xpos = index as i32 * icon_size as i32;
        conn.send_request(&x::ConfigureWindow {
            window,
            value_list: &[
                xcb::x::ConfigWindow::X(xpos),
            ],
        });
    }
}

fn take_ownership(
    conn: &xcb::Connection,
    window: x::Window,
    root: x::Window,
    atoms: Atoms,
    event: &x::PropertyNotifyEvent,
) {
    conn.send_request(&x::SetSelectionOwner {
        owner: window,
        selection: atoms.net_system_tray_s0,
        time: event.time(),
    });

    conn.flush().unwrap();

    let owned = owner(&conn, &atoms) == window;

    if owned {
        conn.send_request(&x::SendEvent {
            propagate: false,
            destination: x::SendEventDest::Window(root),
            event_mask: x::EventMask::STRUCTURE_NOTIFY,
            event: &x::ClientMessageEvent::new(
                root,
                atoms.manager,
                x::ClientMessageData::Data32([
                    event.time(),
                    atoms.net_system_tray_s0.resource_id(),
                    window.resource_id(),
                    0,
                    0,
                ]),
            ),
        });
        conn.flush().unwrap();
    } else {
        error!("problem getting selection");
    }
}

fn is_available(conn: &xcb::Connection, atoms: &Atoms) -> bool {
    owner(conn, atoms).is_none()
}

fn owner(conn: &xcb::Connection, atoms: &Atoms) -> x::Window {
    let cookie = conn.send_request(&x::GetSelectionOwner {
        selection: atoms.net_system_tray_s0,
    });

    let reply = conn.wait_for_reply(cookie).unwrap();

    reply.owner()
}

fn setup_window(
    conn: &xcb::Connection,
    window: x::Window,
    screen: &x::Screen,
    root: x::Window,
    atoms: Atoms,
) {
    conn.send_request(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as _,
        wid: window,
        parent: root,
        x: 2000,
        y: 200,
        width: 400,
        height: 60,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: screen.root_visual(),
        value_list: &[
               x::Cw::BackPixel(screen.black_pixel()),
               x::Cw::OverrideRedirect(true),
               x::Cw::EventMask(x::EventMask::PROPERTY_CHANGE | x::EventMask::BUTTON_PRESS),
        ],
    });

    // window title
    conn.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: x::ATOM_WM_NAME,
        r#type: x::ATOM_STRING,
        data: b"trayproxy",
    });

    // orientation
    conn.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: atoms.net_system_tray_orientation,
        r#type: x::ATOM_CARDINAL,
        data: &[0 as u32],
    });

    // conn.send_request(&x::ChangeProperty {
    //     mode: x::PropMode::Replace,
    //     window,
    //     property: atoms.net_wm_window_type,
    //     r#type: x::ATOM_ATOM,
    //     data: &[atoms.net_wm_window_type_dock],
    // });

    // skip showing in taskbar
    conn.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: atoms.net_wm_state,
        r#type: x::ATOM_ATOM,
        data: &[atoms.net_wm_state_skip_taskbar],
    });

    // Set WM_DELETE_WINDOW protocol
    conn.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window,
        property: atoms.wm_protocols,
        r#type: x::ATOM_ATOM,
        data: &[atoms.wm_delete_window],
    });

    // Append WM_TAKE_FOCUS protocol
    conn.send_request(&x::ChangeProperty {
        mode: x::PropMode::Append,
        window,
        property: atoms.wm_protocols,
        r#type: x::ATOM_ATOM,
        data: &[atoms.wm_take_focus],
    });

    // keeps tray on every workspace screen
    conn.send_request(&x::ChangeProperty {
        mode: x::PropMode::Append,
        window,
        property: atoms.net_wm_state,
        r#type: x::ATOM_ATOM,
        data: &[
            atoms.net_wm_state_sticky,
            atoms.net_wm_state_above,
        ],
    });

    // map
    conn.send_request(&x::MapWindow {
        window,
    });

    conn.flush().unwrap();
}
