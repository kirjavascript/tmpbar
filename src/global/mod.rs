use crate::util::Signal;
use crate::config::ConfigScript;
use crate::wm::xcb::workspaces::Workspaces;

pub struct Global {
    signal: Signal<Event>,
    workspaces: Workspaces,
}

#[derive(Clone, Debug)]
pub enum Event {
    WindowTitle(String),
    WorkspaceCurrent(u32),
    WorkspaceDesktops(Vec<(String, u32, u32)>),
    WorkspaceUrgency(Vec<u32>),
}

// TODO: visible by doing tracking


impl Global {
    pub fn new(ctx: egui::Context) -> Self {
        let signal: Signal<Event> = Signal::new(ctx);

        crate::wm::xcb::listen(signal.clone());

        Self {
            signal,
            workspaces: Workspaces::new(),
        }
    }

    pub fn signals(&mut self, config: &mut ConfigScript) {
        for event in self.signal.read() {
            match event {
                Event::WindowTitle(title) => {
                    config.lua.globals().set("xcake_window_title", title).ok();
                },
                Event::WorkspaceCurrent(current) => {
                    self.workspaces.current = current;
                },
                Event::WorkspaceDesktops(desktops) => {
                    self.workspaces.desktops = desktops;
                },
                Event::WorkspaceUrgency(urgency) => {
                    self.workspaces.urgency = urgency;
                },
            }
        }

        println!("{:?}", self.workspaces);
    }
}
