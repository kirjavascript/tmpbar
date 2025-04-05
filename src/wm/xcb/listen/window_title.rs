use xcb::x;

pub fn handle_event(
    event: &x::PropertyNotifyEvent,
    root: &x::Window,
    conn: &xcb::Connection,
    atoms: &crate::wm::xcb::listen::Atoms,
    active_window: &mut x::Window
) -> Option<String> {
    let atom = event.atom();

    let is_active = atom == atoms.active_window;
    let is_name = atom == x::ATOM_WM_NAME && event.window() == *active_window;

    if is_active {
        let cookie = conn.send_request(&xcb::x::GetProperty {
            delete: false,
            window: *root,
            property: atoms.active_window,
            r#type: xcb::x::ATOM_WINDOW,
            long_offset: 0,
            long_length: 1,
        });

        if let Ok(reply) = conn.wait_for_reply(cookie) {

            let active_windows: &[x::Window] = reply.value();
            if active_windows.len() != 0 {
                *active_window = active_windows[0];
            }
        }
    }

    if is_active || is_name {
        let cookie = conn.send_request(&xcb::x::GetProperty {
            delete: false,
            window: *active_window,
            property: atoms.net_wm_name,
            r#type: atoms.utf8_string,
            long_offset: 0,
            long_length: 256,
        });

        if let Ok(reply) = conn.wait_for_reply(cookie) {
            let value = reply.value();
            let title = String::from_utf8_lossy(value).into_owned();

            return Some(title)
        }
    }

    None
}
