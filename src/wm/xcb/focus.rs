use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};
use xcb::{x, Xid, XidNew};

static LAST_FOCUS: AtomicU32 = AtomicU32::new(0);
static HAS_FOCUS: AtomicBool = AtomicBool::new(false);

pub fn window_focus(id: String, state: bool) {
    std::thread::spawn(move || {
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();

        let windows = super::window::get_windows(&conn, root);

        // window should definitely exist
        let bar = windows.get(&id).unwrap();

        if state {
            if !HAS_FOCUS.load(Ordering::SeqCst) {
                let query_focus_cookie = conn.send_request(&x::GetInputFocus {});
                let query_focus_reply = conn.wait_for_reply(query_focus_cookie).unwrap();
                let current_focus = query_focus_reply.focus();
                let current_focus: u32 = current_focus.resource_id();

                LAST_FOCUS.store(current_focus, Ordering::SeqCst);
                HAS_FOCUS.store(true, Ordering::SeqCst);
            }

            conn.send_request(&x::SetInputFocus {
                revert_to: x::InputFocus::PointerRoot,
                focus: *bar,
                time: x::CURRENT_TIME,
            });
        } else {
            let last_focus = LAST_FOCUS.load(Ordering::SeqCst);
            let last_focus = unsafe { x::Window::new(last_focus) };
            HAS_FOCUS.store(false, Ordering::SeqCst);

            conn.send_request(&x::SetInputFocus {
                revert_to: x::InputFocus::PointerRoot,
                focus: last_focus,
                time: x::CURRENT_TIME,
            });
        }

        conn.flush().unwrap();
    });
}
