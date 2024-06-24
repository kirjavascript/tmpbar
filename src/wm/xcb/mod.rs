mod window;

pub use window::*;

use xcb::x;

pub fn window_focus(id: String, state: bool) {
    std::thread::spawn(move || {
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();

        let mut windows = window::get_windows(&conn, root);

        while windows.get(&id).is_none() {
            windows = window::get_windows(&conn, root);

            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        let bar = windows.get(&id).unwrap();

        if state {
            conn.send_request(&x::SetInputFocus {
                revert_to: x::InputFocus::None,
                focus: *bar,
                time: x::CURRENT_TIME,
            });
        } else {
            conn.send_request(&x::SetInputFocus {
                revert_to: x::InputFocus::None,
                focus: root,
                time: x::CURRENT_TIME,
            });
        }

        conn.flush().unwrap();
    });
}
