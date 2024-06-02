use std::collections::HashMap;

pub fn window_patch() {
    std::thread::spawn(|| {
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();

        let mut windows = get_windows(&conn, root);

        // TODO use highest bar number
        while windows.get("xcake-1").is_none() {
            windows = get_windows(&conn, root);

            std::thread::sleep(std::time::Duration::from_millis(5));
        }


        println!("{:#?}", windows.get("xcake-root"));
        println!("{:#?}", windows.get("xcake-0"));
        println!("{:#?}", windows.get("xcake-1"));

        let cake_root = windows.get("xcake-root").unwrap();
        let cake_0 = windows.get("xcake-0").unwrap();
        let cake_1 = windows.get("xcake-1").unwrap();

        conn.send_request(&xcb::x::UnmapWindow { window: *cake_root });
        conn.flush().unwrap();

        conn.send_request(&xcb::x::ConfigureWindow {
            window: *cake_1,
            value_list: &[
                xcb::x::ConfigWindow::X(0),
                xcb::x::ConfigWindow::Y(1080 - 20),
                xcb::x::ConfigWindow::Width(1920),
                xcb::x::ConfigWindow::Height(20),
            ],
        });

        // conn.flush().unwrap();

        let cookie = conn.send_request(&xcb::x::GetWindowAttributes {
            window: *cake_root,
        });

        let reply = conn.wait_for_reply(cookie).unwrap();
        let value = reply.map_state();
        // let title = String::from_utf8_lossy(value);
        println!("{:#?}", value);


        xcb::atoms_struct! {
            #[derive(Copy, Clone, Debug)]
            pub(crate) struct Atoms {
                // pub windowtype_dock => b"_NET_WM_WINDOW_TYPE_DOCK",
                // pub strut_partial => b"_NET_WM_STRUT_PARTIAL",
                // pub strut => b"_NET_WM_STRUT",
                // pub windowtype => b"_NET_WM_WINDOW_TYPE",
                pub shadow => b"_COMPTON_SHADOW",
            }
        }
        let atoms = Atoms::intern_all(&conn).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(100));

        // picom.conf shadow-exclude "_COMPTON_SHADOW@:32c = 0"

        conn.send_request(&xcb::x::ChangeProperty {
            mode: xcb::x::PropMode::Replace,
            window: *cake_0,
            property: atoms.shadow,
            r#type: xcb::x::ATOM_CARDINAL,
            data: &[0u32],
        });

        conn.send_request(&xcb::x::ChangeProperty {
            mode: xcb::x::PropMode::Replace,
            window: *cake_1,
            property: atoms.shadow,
            r#type: xcb::x::ATOM_ATOM,
            data: &[0u32],
        });

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

        conn.flush().unwrap();
        // conn.send_request(&xcb::x::MapWindow { window: *window });
        // conn.flush().unwrap();

    });
}

fn get_windows(conn: &xcb::Connection, root: xcb::x::Window) -> HashMap<String, xcb::x::Window> {
    let mut windows = HashMap::new();

    fn query(conn: &xcb::Connection, window: xcb::x::Window, windows: &mut HashMap<String, xcb::x::Window>) {
        let cookie = conn.send_request(&xcb::x::QueryTree {
            window,
        });
        let reply = conn.wait_for_reply(cookie).unwrap();

        reply
            .children().iter().for_each(|window| {
                let cookie = conn.send_request(&xcb::x::GetProperty {
                    delete: false,
                    window: *window,
                    property: xcb::x::ATOM_WM_NAME,
                    r#type: xcb::x::ATOM_STRING,
                    long_offset: 0,
                    long_length: 1024,
                });
                let reply = conn.wait_for_reply(cookie).unwrap();
                let value = reply.value();
                let title = String::from_utf8_lossy(value);
                let title = title.trim_end();

                windows.insert(title.to_string(), *window);

                query(&conn, *window, windows);
            });
    }

    query(conn, root, &mut windows);

    windows
}
