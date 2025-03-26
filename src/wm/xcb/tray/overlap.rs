use xcb::{x, Connection, xproto};

fn listen(tray_window: x::Window) {
    let (conn, screen_num) = Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let values = [(xproto::CW_EVENT_MASK,
        xproto::EVENT_MASK_SUBSTRUCTURE_NOTIFY |
        xproto::EVENT_MASK_STRUCTURE_NOTIFY)
    ];

    xproto::change_window_attributes(&conn, screen.root(), &values);
    conn.flush();

    loop {
        let event = conn.wait_for_event();
        match event {
            Ok(Event::X(x::Event::ConfigureNotify(event))) => {
                println!("ConfigureNotify: window = {:?}", event.window());
                if is_window_overlapping(&conn, event.window()) {
                    println!("Overlap detected!");
                }
            }
            Ok(Event::X(x::Event::MapNotify(event))) => {
                println!("MapNotify: window = {:?}", event.window());
                if is_window_overlapping(&conn, event.window()) {
                    println!("Overlap detected!");
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
                subscribe_events(&conn, &event.window());
            }
            _ => {}
        }
    }
}

fn is_window_overlapping(conn: &Connection, window: u32) -> bool {
    let setup = conn.get_setup();
    let root = setup.roots().next().unwrap().root;

    // Query the window tree
    let tree = xproto::query_tree(&conn, root).get_reply().unwrap();
    let stack = tree.children();

    // Get the target window's geometry
    let target_geometry = xproto::get_geometry(&conn, window).get_reply().unwrap();

    for &child in stack {
        if child == window {
            continue;
        }

        let child_geometry = xproto::get_geometry(&conn, child).get_reply().unwrap();

        if rectangles_overlap(
            target_geometry.x(), target_geometry.y(), target_geometry.width(), target_geometry.height(),
            child_geometry.x(), child_geometry.y(), child_geometry.width(), child_geometry.height(),
        ) {
            return true;
        }
    }

    false
}

fn rectangles_overlap(
    x1: i16, y1: i16, w1: u16, h1: u16,
    x2: i16, y2: i16, w2: u16, h2: u16
) -> bool {
    x1 < x2 + w2 as i16 &&
        x1 + w1 as i16 > x2 &&
        y1 < y2 + h2 as i16 &&
        y1 + h1 as i16 > y2
}
