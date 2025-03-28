use xcb::{x, Event, Connection};
use std::collections::HashSet;
use crossbeam_channel::Sender;

xcb::atoms_struct! {
    #[derive(Copy, Clone, Debug)]
    pub(crate) struct Atoms {
        pub wm_type => b"_NET_WM_WINDOW_TYPE",
        pub dock => b"_NET_WM_WINDOW_TYPE_DOCK",
        pub popup_menu => b"_NET_WM_WINDOW_TYPE_POPUP_MENU",
        pub utf8_string => b"ATOM_UTF8_STRING",
    }
}

pub fn listen(tray_window: x::Window, tx: Sender<bool>) {
    let (conn, screen_num) = Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let atoms = Atoms::intern_all(&conn).unwrap();
    let root = screen.root();

    let change_attrs = x::ChangeWindowAttributes {
        window: screen.root(),
        value_list: &[
            x::Cw::EventMask(
                x::EventMask::STRUCTURE_NOTIFY
                | x::EventMask::SUBSTRUCTURE_NOTIFY
            ),
        ],
    };

    conn.send_request_checked(&change_attrs);
    conn.flush().ok();

    subscribe_windows(&conn, root);

    let mut overlaps = HashSet::new();
    let mut is_overlapping = false;

    loop {
        let event = conn.wait_for_event();
        match event {
            Ok(Event::X(x::Event::ConfigureNotify(event))) => {
                let window = event.window();

                if is_window_overlapping(&conn, &atoms, root, window, tray_window) {
                    overlaps.insert(window);
                } else {
                    overlaps.remove(&window);
                }
            }
            Ok(Event::X(x::Event::MapNotify(event))) => {
                let window = event.window();

                if is_window_overlapping(&conn, &atoms, root, window, tray_window) {
                    overlaps.insert(window);
                } else {
                    overlaps.remove(&window);
                }
            }
            Ok(Event::X(x::Event::UnmapNotify(event))) => {
                overlaps.remove(&event.window());
            }
            Ok(Event::X(x::Event::DestroyNotify(event))) => {
                overlaps.remove(&event.window());
            }
            Ok(Event::X(x::Event::CreateNotify(event))) => {
                subscribe_events(&conn, event.window());
            }
            _ => {}
        }

        if overlaps.is_empty() {
            if is_overlapping == true {
                is_overlapping = false;
                tx.send(false).ok();
            }
        } else {
            if is_overlapping == false {
                is_overlapping = true;
                tx.send(true).ok();
            }
        }
    }
}

fn subscribe_events(conn: &Connection, window: x::Window) {
    let change_attrs = x::ChangeWindowAttributes {
        window,
        value_list: &[
            x::Cw::EventMask(
                x::EventMask::STRUCTURE_NOTIFY
                | x::EventMask::SUBSTRUCTURE_NOTIFY
                | x::EventMask::EXPOSURE
            ),
        ],
    };

    conn.send_request(&change_attrs);
    conn.flush().ok();
}

fn subscribe_windows(conn: &xcb::Connection, window: x::Window) {
    subscribe_events(conn, window);

    let cookie = conn.send_request(&xcb::x::QueryTree {
        window,
    });

    if let Ok(tree) = conn.wait_for_reply(cookie) {
        for child in tree.children() {
            subscribe_windows(conn, *child);
        }
    }
}

fn is_window_mapped(conn: &Connection, window: x::Window) -> bool {
    let attrs_cookie = conn.send_request(&x::GetWindowAttributes { window });
    let attrs_reply = conn.wait_for_reply(attrs_cookie);

    if let Ok(attrs) = attrs_reply {
        return attrs.map_state() == x::MapState::Viewable;
    }

    false
}

fn ignore_wm_type(conn: &Connection, atoms: &Atoms, window: x::Window) -> bool {
    let prop_cookie = conn.send_request(&x::GetProperty {
        delete: false,
        window,
        property: atoms.wm_type,
        r#type: x::ATOM_ATOM,
        long_offset: 0,
        long_length: 32,
    });

    let prop_reply = conn.wait_for_reply(prop_cookie);

    if let Ok(prop) = prop_reply {
        let win_atoms = prop.value::<x::Atom>();

        // ignore windows without a WM_TYPE
        if win_atoms.is_empty() {
            return true
        }

        // ignore docks and popup menus
        for atom in win_atoms {
            if *atom == atoms.dock || *atom == atoms.popup_menu {
                return true
            }
        }

        false
    } else {
        true
    }
}

fn is_window_overlapping(
    conn: &Connection,
    atoms: &Atoms,
    root: x::Window,
    window: x::Window,
    tray: x::Window,
) -> bool {
    if window == tray {
        return false
    }
    if !is_window_mapped(conn, window) {
        return false
    }
    if ignore_wm_type(conn, atoms, window) {
        return false
    }

    let target_geom_cookie = conn.send_request(&x::GetGeometry { drawable: x::Drawable::Window(window) });
    let target_geometry_result = conn.wait_for_reply(target_geom_cookie);

    if target_geometry_result.is_err() {
        return false
    }

    let target_geometry = target_geometry_result.unwrap();

    let tray_geom_cookie = conn.send_request(&x::GetGeometry { drawable: x::Drawable::Window(tray) });
    let tray_geometry_result = conn.wait_for_reply(tray_geom_cookie);

    if tray_geometry_result.is_err() {
        return false
    }

    let tray_geometry = tray_geometry_result.unwrap();

    // get absolute coordinates

    let target_translate_cookie = conn.send_request(&x::TranslateCoordinates {
        src_window: window,
        dst_window: root,
        src_x: 0,
        src_y: 0,
    });

    let translate_result = conn.wait_for_reply(target_translate_cookie);

    let (target_x, target_y) = if let Ok(ref translate) = translate_result {
        (translate.dst_x(), translate.dst_y())
    } else {
        (target_geometry.x(), target_geometry.y())
    };

    let tray_translate_cookie = conn.send_request(&x::TranslateCoordinates {
        src_window: tray,
        dst_window: root,
        src_x: 0,
        src_y: 0,
    });

    let translate_result = conn.wait_for_reply(tray_translate_cookie);

    let (tray_x, tray_y) = if let Ok(ref translate) = translate_result {
        (translate.dst_x(), translate.dst_y())
    } else {
        (tray_geometry.x(), tray_geometry.y())
    };

    return rectangles_overlap(
        target_x, target_y, target_geometry.width(), target_geometry.height(),
        tray_x, tray_y, tray_geometry.width(), tray_geometry.height(),
    );
}

fn rectangles_overlap(
    x1: i16, y1: i16, w1: u16, h1: u16,
    x2: i16, y2: i16, w2: u16, h2: u16
) -> bool {
    if w1 == 0 || h1 == 0 || w2 == 0 || h2 == 0 {
        return false;
    }

    x1 < x2 + w2 as i16 &&
    x1 + w1 as i16 > x2 &&
    y1 < y2 + h2 as i16 &&
    y1 + h1 as i16 > y2
}
