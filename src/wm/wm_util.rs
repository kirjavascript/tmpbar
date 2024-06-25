use crate::util::Signal;

use std::cell::RefCell;
use std::rc::Rc;

pub struct WMUtil(Rc<RefCell<WMUtilInner>>);

struct WMUtilInner {
    window_title: String,
    ctx: egui::Context,
}

impl WMUtil {
    pub fn new(ctx: egui::Context) -> Self {

        xcb::atoms_struct! {
            #[derive(Copy, Clone, Debug)]
            pub(crate) struct Atoms {
                // pub strut_partial => b"_NET_WM_STRUT_PARTIAL",
                // pub strut => b"_NET_WM_STRUT",
                pub shadow => b"_COMPTON_SHADOW",
            }
        }

        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        let root = screen.root();
        let atoms = Atoms::intern_all(&conn).unwrap();


        std::thread::spawn(move || {


        });

        WMUtil(Rc::new(RefCell::new(WMUtilInner {
            window_title: "".to_string(),
            ctx,
        })))
    }
}
