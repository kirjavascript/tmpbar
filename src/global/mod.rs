mod lua;

use crate::util::Signal;
use crate::wm::xcb::workspaces::Workspaces;
use crate::wm::xcb;
use crate::wm::i3mode;
use lua::LuaCallback;

pub struct Global {
    pub lua: mlua::Lua,
    pub workspaces: Workspaces,
    pub tray: Option<xcb::Tray>,
    pub parent_path: String,
    xcb_signal: Signal<xcb::Event>,
    lua_signal: Signal<LuaCallback>,
    i3mode_signal: Signal<String>
}

impl Global {
    pub fn new(path: &std::path::PathBuf, ctx: egui::Context) -> Self {
        let xcb_signal: Signal<xcb::Event> = Signal::new(ctx.clone());
        xcb::listen(xcb_signal.clone());

        // set parent path
        let parent = path.parent().map(|p| p.to_path_buf());
        let parent_path = format!("{}/", parent.expect("error getting parent path").to_string_lossy());

        if let Err(error) = std::env::set_current_dir(&parent_path) {
            error!("cannot set cwd {}", error);
        }

        let (lua, lua_signal) = lua::load_lua(ctx.clone());

        let i3mode_signal: Signal<String> = Signal::new(ctx);
        i3mode::listen(i3mode_signal.clone()).ok();

        Self {
            workspaces: Workspaces::new(),
            tray: None,
            parent_path,
            lua,
            xcb_signal,
            lua_signal,
            i3mode_signal,
        }
    }

    pub fn signals(&mut self) {
        for event in self.xcb_signal.read() {
            match event {
                xcb::Event::WindowTitle(title) => {
                    self.lua.globals().set("xcake_window_title", title).ok();
                }
                xcb::Event::WorkspaceCurrent(current) => {
                    self.workspaces.set_current(current);
                }
                xcb::Event::WorkspaceDesktops(desktops) => {
                    self.workspaces.set_desktops(desktops);
                }
                xcb::Event::WorkspaceUrgency(urgency) => {
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

        for mode in self.i3mode_signal.read() {
            self.lua.globals().set("xcake_i3_mode", mode).ok();
        }

        if let Some(tray) = self.tray.as_mut() {
            tray.signals();
        }
    }
}
