use crate::util::Signal;
use crate::wm::xcb::workspaces::Workspaces;

pub struct Global {
    pub lua: mlua::Lua,
    pub workspaces: Workspaces,
    signal: Signal<Event>,
}

#[derive(Clone, Debug)]
pub enum Event {
    WindowTitle(String),
    WorkspaceCurrent(u32),
    WorkspaceDesktops(Vec<(String, u32, u32)>),
    WorkspaceUrgency(Vec<u32>),
}

impl Global {
    pub fn new(ctx: egui::Context) -> Self {
        let signal: Signal<Event> = Signal::new(ctx);

        crate::wm::xcb::listen(signal.clone());

        let lua = mlua::Lua::new();

        lua.load(include_str!("./prelude.lua")).exec().unwrap();

        Self {
            workspaces: Workspaces::new(),
            lua,
            signal,
        }
    }

    pub fn signals(&mut self) {
        for event in self.signal.read() {
            match event {
                Event::WindowTitle(title) => {
                    self.lua.globals().set("xcake_window_title", title).ok();
                },
                Event::WorkspaceCurrent(current) => {
                    self.workspaces.set_current(current);
                },
                Event::WorkspaceDesktops(desktops) => {
                    self.workspaces.set_desktops(desktops);
                },
                Event::WorkspaceUrgency(urgency) => {
                    self.workspaces.set_urgency(urgency);
                },
            }
        }
    }
}
