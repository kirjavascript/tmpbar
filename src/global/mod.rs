use crate::util::Signal;
use crate::wm::xcb::workspaces::Workspaces;

pub struct Global {
    pub lua: mlua::Lua,
    pub workspaces: Workspaces,
    pub parent_path: String,
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
    pub fn new(path: &str, ctx: egui::Context) -> Self {
        let signal: Signal<Event> = Signal::new(ctx);

        crate::wm::xcb::listen(signal.clone());

        let lua = load_lua(path);

        let parent_path = lua.globals().get("xcake_parent_path").unwrap_or_default();

        Self {
            workspaces: Workspaces::new(),
            parent_path,
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

fn load_lua(path: &str) -> mlua::Lua {
    let lua = mlua::Lua::new();
    lua.load(include_str!("./prelude.lua")).exec().unwrap();

    // save parent path
    if let Ok(path) = std::fs::canonicalize(std::path::Path::new(path)) {
        let parent = path.parent().map(|p| p.to_path_buf());
        lua.globals().set("xcake_parent_path", parent.unwrap().to_string_lossy() + "/").ok();
    }

    lua
}
