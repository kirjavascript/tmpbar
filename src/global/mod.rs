use crate::util::Signal;

pub struct Global {
    signal: Signal<Event>,
}

#[derive(Clone)]
pub enum Event {
    WindowTitle(String),
}

impl Global {
    pub fn new(ctx: egui::Context) -> Self {
        let signal: Signal<Event> = Signal::new(ctx);

        crate::wm::xcb::listen(signal.clone());

        Self {
            signal,
        }
    }

    pub fn signals(&self, lua: &mlua::Lua) {
        for event in self.signal.read() {
            match event {
                Event::WindowTitle(title) => {
                    lua.globals().set("xcake_window_title", title).ok();
                },
            }
        }
    }
}
