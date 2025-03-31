mod lua;

use crate::util::Signal;
use crate::wm::xcb::workspaces::Workspaces;
use crate::wm::xcb::{Event, Tray};
use crate::wm::i3mode::I3Mode;
use lua::LuaCallback;

pub struct Global {
    pub lua: mlua::Lua,
    pub workspaces: Workspaces,
    pub tray: Option<Tray>,
    pub parent_path: String,
    pub i3mode: I3Mode,
    xcb_signal: Signal<Event>,
    lua_signal: Signal<LuaCallback>,
}

impl Global {
    pub fn new(path: &str, ctx: egui::Context) -> Self {
        let xcb_signal: Signal<Event> = Signal::new(ctx.clone());
        crate::wm::xcb::listen(xcb_signal.clone());

        let (lua, lua_signal) = lua::load_lua(path, ctx.clone());

        let i3mode = I3Mode::new(ctx);

        let parent_path = lua.globals().get("xcake_parent_path").unwrap_or_default();

        Self {
            workspaces: Workspaces::new(),
            tray: None,
            parent_path,
            lua,
            i3mode,
            xcb_signal,
            lua_signal,
        }
    }

    pub fn signals(&mut self) {
        for event in self.xcb_signal.read() {
            match event {
                Event::WindowTitle(title) => {
                    self.lua.globals().set("xcake_window_title", title).ok();
                }
                Event::WorkspaceCurrent(current) => {
                    self.workspaces.set_current(current);
                }
                Event::WorkspaceDesktops(desktops) => {
                    self.workspaces.set_desktops(desktops);
                }
                Event::WorkspaceUrgency(urgency) => {
                    self.workspaces.set_urgency(urgency);
                }
            }
        }

        for event in self.lua_signal.read() {
            match event {
                LuaCallback::CycleWorkspace(direction) => {
                    self.workspaces.cycle_workspace(direction);
                },
                LuaCallback::FocusWorkspace(desktop) => {
                    self.workspaces.focus_workspace(desktop);
                },
            }
        }

        if let Some(tray) = self.tray.as_mut() {
            tray.signals();
        }
    }
}
