mod atoms;
mod window;
mod focus;
mod listen;

use atoms::Atoms;
pub use focus::*;
pub use window::*;
pub use listen::*;


#[allow(dead_code)]
fn debug_title() {
    use xcb::x;
    std::thread::spawn(|| {
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();

    // Create a window
    let window = conn.generate_id();

    // We can now create a window. For this we pass a `Request`
    // object to the `send_request_checked` method. The method
    // returns a cookie that will be used to check for success.
    let cookie = conn.send_request_checked(&x::CreateWindow {
        depth: x::COPY_FROM_PARENT as u8,
        wid: window,
        parent: screen.root(),
        x: 0,
        y: 0,
        width: 150,
        height: 150,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: screen.root_visual(),
        // this list must be in same order than `Cw` enum order
        value_list: &[
            x::Cw::BackPixel(screen.white_pixel()),
            x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS)
        ],
    });
    // We now check if the window creation worked.
    // A cookie can't be cloned; it is moved to the function.
    conn.check_request(cookie).unwrap();

 conn.send_request(&x::MapWindow {
        window,
    });
    conn.flush().unwrap();

    // Loop to update the window title every second
    let mut count = 0;
    loop {
        let title = format!("Title #{}", count);
        let cookie = conn.send_request_checked(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window,
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: title.as_bytes(),
        });
conn.check_request(cookie).unwrap();
        conn.flush().unwrap();
        count += 1;
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    });
}
