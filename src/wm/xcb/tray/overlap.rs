use xcb::{x, Event, Connection};

pub fn listen(tray_window: x::Window) {
    let (conn, screen_num) = Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

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

    let mut overlaps = std::collections::HashMap::new();

    let wm_type_cookie = conn.send_request(&x::InternAtom {
        only_if_exists: true,
        name: b"ATOM_UTF8_STRING",
    });

    let utf8_string = conn.wait_for_reply(wm_type_cookie).unwrap();

    // TODO: iterate through all windows and check if any overlap at start

    loop {
        let event = conn.wait_for_event();
        match event {
            Ok(Event::X(x::Event::ConfigureNotify(event))) => {
                let window = event.window();

                let cookie = conn.send_request(&xcb::x::GetProperty {
                    delete: false,
                    window,
                    property: xcb::x::ATOM_WM_NAME,
                    r#type: utf8_string.atom(),
                    long_offset: 0,
                    long_length: 256,
                });

                let title = if let Ok(reply) = conn.wait_for_reply(cookie) {
                    let value = reply.value();
                    let title = String::from_utf8_lossy(value).into_owned();

                    title
                } else {
                    "...".to_string()
                };

                if is_window_overlapping(&conn, window, tray_window) {
                    overlaps.insert(window, title);
                } else {
                    overlaps.remove(&window);
                }
            }
            Ok(Event::X(x::Event::MapNotify(event))) => {
                let window = event.window();

                let cookie = conn.send_request(&xcb::x::GetProperty {
                    delete: false,
                    window,
                    property: xcb::x::ATOM_WM_NAME,
                    r#type: utf8_string.atom(),
                    long_offset: 0,
                    long_length: 256,
                });

                let title = if let Ok(reply) = conn.wait_for_reply(cookie) {
                    let value = reply.value();
                    let title = String::from_utf8_lossy(value).into_owned();

                    title
                } else {
                    "...".to_string()
                };

                if is_window_overlapping(&conn, window, tray_window) {
                    overlaps.insert(window, title);
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
        println!("{:#?}", overlaps);
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

    conn.send_request_checked(&change_attrs);
    conn.flush().ok();
}

fn is_override_redirect(conn: &Connection, window: x::Window) -> bool {
    let attrs_cookie = conn.send_request(&x::GetWindowAttributes { window });
    let attrs_reply = conn.wait_for_reply(attrs_cookie);

    if let Ok(attrs) = attrs_reply {
        return attrs.override_redirect();
    }

    false
}

fn is_window_mapped(conn: &Connection, window: x::Window) -> bool {
    let attrs_cookie = conn.send_request(&x::GetWindowAttributes { window });
    let attrs_reply = conn.wait_for_reply(attrs_cookie);

    if let Ok(attrs) = attrs_reply {
        return attrs.map_state() == x::MapState::Viewable;
    }

    false
}

fn is_dock_window(conn: &Connection, window: x::Window) -> bool {
    // Get the atom for _NET_WM_WINDOW_TYPE
    let wm_type_cookie = conn.send_request(&x::InternAtom {
        only_if_exists: true,
        name: b"_NET_WM_WINDOW_TYPE",
    });

    // Get the atom for _NET_WM_WINDOW_TYPE_DOCK
    let dock_type_cookie = conn.send_request(&x::InternAtom {
        only_if_exists: true,
        name: b"_NET_WM_WINDOW_TYPE_DOCK",
    });

    let wm_type_reply = conn.wait_for_reply(wm_type_cookie);
    let dock_type_reply = conn.wait_for_reply(dock_type_cookie);

    if wm_type_reply.is_err() || dock_type_reply.is_err() {
        return false;
    }

    let wm_type_atom = wm_type_reply.unwrap().atom();
    let dock_type_atom = dock_type_reply.unwrap().atom();

    let prop_cookie = conn.send_request(&x::GetProperty {
        delete: false,
        window,
        property: wm_type_atom,
        r#type: x::ATOM_ATOM,
        long_offset: 0,
        long_length: 32,
    });

    let prop_reply = conn.wait_for_reply(prop_cookie);

    if let Ok(prop) = prop_reply {
        let atoms = prop.value::<x::Atom>();
        for atom in atoms {
            if *atom == dock_type_atom {
                return true;
            }
        }
    }

    false
}

fn is_window_overlapping(conn: &Connection, window: x::Window, tray: x::Window) -> bool {
    if window == tray {
        return false
    }
    // if is_override_redirect(conn, window) {
    //     return false
    // }
    if is_dock_window(conn, window) {
        return false
    }

    let target_geom_cookie = conn.send_request(&x::GetGeometry { drawable: x::Drawable::Window(window) });
    let target_geometry_result = conn.wait_for_reply(target_geom_cookie);

    if target_geometry_result.is_err() {
        return false
    }

    let target_geometry = target_geometry_result.unwrap();

    let child_geom_cookie = conn.send_request(&x::GetGeometry { drawable: x::Drawable::Window(tray) });
    let child_geometry_result = conn.wait_for_reply(child_geom_cookie);

    if child_geometry_result.is_err() {
        return false
    }

    let child_geometry = child_geometry_result.unwrap();

    return rectangles_overlap(
        target_geometry.x(), target_geometry.y(), target_geometry.width(), target_geometry.height(),
        child_geometry.x(), child_geometry.y(), child_geometry.width(), child_geometry.height(),
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
