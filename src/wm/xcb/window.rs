use std::collections::HashMap;
use crate::config::ConfigScript;
use crate::wm::monitor::Monitor;

xcb::atoms_struct! {
    #[derive(Copy, Clone, Debug)]
    pub(crate) struct Atoms {
        pub shadow => b"_COMPTON_SHADOW",
        // pub strut_partial => b"_NET_WM_STRUT_PARTIAL",
        // pub strut => b"_NET_WM_STRUT",
    }
}

struct SendBar {
    id: String,
    y: i32,
    height: u32,
    monitor: Monitor,
}

pub fn window_patch(config: &ConfigScript) {
    if config.bars.len() == 0 {
        return
    }

    let bars: Vec<SendBar> = config.bars.iter().map(|bar| SendBar {
        id: bar.id(),
        y: bar.y(),
        height: bar.height,
        monitor: bar.monitor.clone(),
    }).collect();

    std::thread::spawn(move || {
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();
        let atoms = Atoms::intern_all(&conn).unwrap();

        let mut windows = get_windows(&conn, root);

        // it is completely ridiculous to use window titles to find windows,
        // but to get a direct X window reference we have to fork eframe
        //
        // ... at least we only have to do this once
        while windows.get(&bars.last().unwrap().id).is_none() {
            windows = get_windows(&conn, root);

            std::thread::sleep(std::time::Duration::from_millis(50));
        }


        let cake_root = windows.get("xcake-root").unwrap();

        conn.send_request(&xcb::x::UnmapWindow { window: *cake_root });
        conn.flush().unwrap();

        for bar in bars.iter() {
            let window = windows.get(&bar.id).unwrap();

            conn.send_request(&xcb::x::ConfigureWindow {
                window: *window,
                value_list: &[
                    xcb::x::ConfigWindow::X(bar.monitor.x as _),
                    xcb::x::ConfigWindow::Y(bar.y as _),
                    xcb::x::ConfigWindow::Width(bar.monitor.width as _),
                    xcb::x::ConfigWindow::Height(bar.height as _),
                ],
            });

            conn.flush().unwrap();

            std::thread::sleep(std::time::Duration::from_millis(100));

            // picom.conf shadow-exclude "_COMPTON_SHADOW@:32c = 0"

            conn.send_request(&xcb::x::ChangeProperty {
                mode: xcb::x::PropMode::Replace,
                window: *window,
                property: atoms.shadow,
                r#type: xcb::x::ATOM_CARDINAL,
                data: &[0u32],
            });

            conn.flush().unwrap();

        }


        // TODO: finish STRUT
        // let width = 1920;
        // let height = 1080;


        // let data =
        //     [
        //      0u32, 0, 0, 0, 21, 0, 0, 0, 0, 0, 1920, 3840
        //     ];

        // let data2 = [
        //     0u32, 0, 0, 21
        // ];


        // conn.send_request(&xcb::x::ChangeProperty {
        //     mode: xcb::x::PropMode::Replace,
        //     window: *window,
        //     property: atoms.strut,
        //     r#type: xcb::x::ATOM_CARDINAL,
        //     data: &data2,
        // });

        // conn.send_request(&xcb::x::ChangeProperty {
        //     mode: xcb::x::PropMode::Replace,
        //     window: *window,
        //     property: atoms.windowtype,
        //     r#type: xcb::x::ATOM_ATOM,
        //     data: &[atoms.windowtype_dock],
        // });
    });
}

pub fn get_windows(conn: &xcb::Connection, root: xcb::x::Window) -> HashMap<String, xcb::x::Window> {
    let mut windows = HashMap::new();

    fn query(conn: &xcb::Connection, window: xcb::x::Window, windows: &mut HashMap<String, xcb::x::Window>) {
        let cookie = conn.send_request(&xcb::x::QueryTree {
            window,
        });
        let reply = conn.wait_for_reply(cookie).unwrap();

        reply
            .children().iter().for_each(|window| {
                if let Some(name) = get_wm_name(&conn, &window) {
                    let title = name.trim_end();

                    windows.insert(title.to_string(), *window);
                }

                query(&conn, *window, windows);
            });
    }

    query(conn, root, &mut windows);

    windows
}

pub fn get_wm_name(conn: &xcb::Connection, window: &xcb::x::Window) -> Option<String> {
    let cookie = conn.send_request(&xcb::x::GetProperty {
        delete: false,
        window: *window,
        property: xcb::x::ATOM_WM_NAME,
        r#type: xcb::x::ATOM_STRING,
        long_offset: 0,
        long_length: 4,
    });

    if let Ok(reply) = conn.wait_for_reply(cookie) {
        let value = reply.value();
        Some(String::from_utf8_lossy(value).into_owned())
    } else {
        None
    }
}
