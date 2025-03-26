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

    loop {
        let event = conn.wait_for_event();
        match event {
            Ok(Event::X(x::Event::ConfigureNotify(event))) => {
                if is_window_overlapping(&conn, event.window(), tray_window) {
                    println!("Overlap1");
                } else {
                    println!("no overlap1");
                }
            }
            Ok(Event::X(x::Event::MapNotify(event))) => {
                if is_window_overlapping(&conn, event.window(), tray_window) {
                    println!("Overlap2");
                } else {
                    println!("no overlap2");
                }
            }
            Ok(Event::X(x::Event::UnmapNotify(event))) => {
                println!("UnmapNotify: window = {:?}", event.window());
            }
            Ok(Event::X(x::Event::DestroyNotify(event))) => {
                println!("DestroyNotify: window = {:?}", event.window());
            }
            Ok(Event::X(x::Event::CreateNotify(event))) => {
                println!("CreateNotify: window = {:?}", event.window());
                subscribe_events(&conn, event.window());
            }
            _ => {}
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

fn is_window_overlapping(conn: &Connection, window: x::Window, tray: x::Window) -> bool {
    // if is_override_redirect(conn, window) {
    //     return false
    // }

    // TODO: handle the bar

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
