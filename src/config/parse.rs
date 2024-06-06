use crate::wm::monitor;

#[derive(Clone)]
pub struct ConfigScript {
    pub path: String,
    pub bars: Vec<Bar>,
}

#[derive(Clone, PartialEq)]
pub enum Position {
    Top,
    Bottom,
}

#[derive(Clone)]
pub struct Bar {
    pub position: Position,
    pub height: i32,
    pub monitor: monitor::Monitor,
}

impl Bar {
    pub fn y(&self) -> i32 {

        if self.position == Position::Top {
            0
        } else {
            self.monitor.height - self.height
        }
    }
}

pub fn parse_script(path: &str, lua: &mlua::Lua) -> mlua::Result<ConfigScript> {
    let monitors = monitor::list();
    let globals = lua.globals();

    let xcake_bars: mlua::Table = globals.get("xcake_bars")?;
    let mut bars = Vec::new();

    for pair in xcake_bars.pairs::<i32, mlua::Table>() {
        let (_, value) = pair?;

        let position: String = value.get("position").unwrap_or_else(|_| "top".to_string());
        let position = if position == "top".to_string() { Position::Top } else { Position::Bottom };
        let height: i32 = value.get("height").unwrap_or(25);

        let empty_table = lua.create_table()?;
        let monitor: mlua::Table = value.get("monitor").unwrap_or_else(|_| empty_table);
        let monitor_name: Result<String, mlua::prelude::LuaError> = monitor.get("name");
        let monitor_index = monitor.get("index").unwrap_or(1);
        let monitor = if let Ok(name) = monitor_name {
            monitors.iter().find(|monitor| { monitor.name == name })
        } else {
            None
        };
        let monitor = monitor.unwrap_or_else(|| {
            monitors.get(monitor_index - 1).unwrap_or(&monitors[0])
        });

        bars.push(Bar {
            position,
            height,
            monitor: monitor.clone(),
        });
    }

    Ok(ConfigScript {
        path: path.to_string(),
        bars,
    })
}
