mod lua;
pub mod theme;

use crate::util::Signal;
use crate::wm::xcb::workspaces::Workspaces;
use crate::wm::xcb;
use crate::wm::i3mode;
use lua::LuaCallback;
pub use theme::Theme;

use eframe::glow;
use std::sync::Arc;

pub struct Global {
    pub lua: mlua::Lua,
    pub workspaces: Workspaces,
    pub tray: Option<xcb::Tray>,
    pub parent_path: String,
    pub theme: Theme,
    pub ctx: egui::Context,
    pub gl: Arc<glow::Context>,
    xcb_signal: Signal<xcb::Event>,
    lua_signal: Signal<LuaCallback>,
    i3mode_signal: Signal<String>
}

impl Global {
    pub fn new(
        path: &std::path::PathBuf,
        ctx: egui::Context,
        gl: Arc<glow::Context>,
    ) -> Self {
        let xcb_signal: Signal<xcb::Event> = Signal::new(ctx.clone());
        xcb::listen(xcb_signal.clone());

        // set parent path
        let parent = path.parent().map(|p| p.to_path_buf());
        let parent_path = format!("{}/", parent.expect("error getting parent path").to_string_lossy());

        if let Err(error) = std::env::set_current_dir(&parent_path) {
            error!("cannot set cwd {}", error);
        }

        let (lua, lua_signal) = lua::load_lua(ctx.clone());

        let i3mode_signal: Signal<String> = Signal::new(ctx.clone());
        i3mode::listen(i3mode_signal.clone()).ok();

        Self {
            workspaces: Workspaces::new(),
            tray: None,
            parent_path,
            lua,
            theme: Theme::default(),
            xcb_signal,
            lua_signal,
            i3mode_signal,
            ctx,
            gl,
        }
    }

    pub fn frame(&mut self) {
        self.set_theme_families();
        self.signals();
    }

    fn signals(&mut self) {
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

    pub fn set_theme_families(&mut self) {
        let mut fonts = self.ctx.fonts(|f| f.families()).iter().map(|font| {
            format!("{}", font)
        }).collect::<Vec<String>>()[2..].to_vec();

        fonts.push("monospace".to_string());

        self.theme.families = fonts;
    }

    // on config load
    pub fn set_theme(&mut self, config: &crate::config::ConfigScript) {
        use crate::config::Property;

        self.theme = Default::default();

        if let Some(bar) = config.bars.get(0) {
            if let Some(Property::Object(style)) = bar.container.props_ref().get("style") {
                if let Some(Property::String(color)) = style.get("color") {
                    match crate::util::color_parse(color) {
                        Ok(color) => {
                            let mut visuals = egui::Visuals::default();
                            visuals.override_text_color = Some(color);
                            self.ctx.set_visuals(visuals);

                            self.theme.color = color;
                        },
                        Err(err) => error!("{}", err),
                    }
                }

                if let Some(Property::String(family)) = style.get("font_family") {
                    self.theme.font_family = Some(family.clone());
                }

                if let Some(Property::Integer(size)) = style.get("font_size") {
                    self.theme.font_size = *size as _;
                } else if let Some(Property::Float(size)) = style.get("font_size") {
                    self.theme.font_size = *size as _;
                }
            }
        }
    }

    pub fn resolve_path(&self, path: &str) -> String {
        if path.starts_with("file://")
            || path.starts_with("http://")
            || path.starts_with("https://")
            || path.starts_with("/") {
                path.to_owned()
        } else {
            format!("file://{}{}", self.parent_path, path)
        }
    }
}
