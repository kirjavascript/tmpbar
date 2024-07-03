use std::fs::{File, canonicalize};
use std::path::Path;
use std::io::prelude::*;
use super::parse::{Bar, parse_bars};
use crate::util::Signal;

pub struct ConfigScript {
    pub path: String,
    pub bars: Vec<Bar>,
    pub reload_signal: Signal<()>,
}

impl ConfigScript {
    pub fn reload(&mut self, lua: &mlua::Lua) -> Result<(), String> {
        load(self, lua)?;
        Ok(())
    }
}

pub fn init(path: &str, ctx: egui::Context, lua: &mlua::Lua) -> ConfigScript {
    let mut script = ConfigScript {
        path: path.to_owned(),
        bars: Vec::new(),
        reload_signal: Signal::new(ctx),
    };


    let load_result = load(&mut script, lua);

    match load_result {
        Ok(_) => {
            super::watch::init(path, script.reload_signal.clone());

            script
        },
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(0);
        }
    }
}

fn load(script: &mut ConfigScript, lua: &mlua::Lua) -> Result<(), String> {
    let code = {
        let path = std::path::Path::new(&script.path);

        let mut file_result = File::open(path).map_err(|x| {
            format!("{}: {}", path.display(), x.to_string())
        })?;

        let mut script = String::new();
        file_result
            .read_to_string(&mut script)
            .map_err(|x| x.to_string())?;

        script
    };

    eval(script, lua, code).map_err(|err| err.to_string())
}

fn eval(script: &mut ConfigScript, lua: &mlua::Lua, code: String) -> mlua::Result<()> {
    let globals = lua.globals();

    if let Ok(path) = canonicalize(Path::new(&script.path)) {
        let parent = path.parent().map(|p| p.to_path_buf());
        globals.set("xcake_parent_path", parent.unwrap().to_string_lossy() + "/")?;
    }

    set_monitors(lua, &globals)?;

    let set_state: mlua::Function = lua.globals().get("xcake_reset_state")?;
    set_state.call(())?;

    lua.load(code).exec()?;

    script.bars = parse_bars(lua)?;

    Ok(())
}

fn set_monitors(lua: &mlua::Lua, globals: &mlua::Table) -> mlua::Result<()> {
    let monitors = lua.create_table()?;
    let monitor_list = crate::wm::monitor::list();

    for (i, monitor) in monitor_list.iter().enumerate() {
        let monitor_table = lua.create_table()?;

        monitor_table.set("index", i + 1)?;
        monitor_table.set("name", monitor.name.to_string())?;

        monitors.set(i + 1, monitor_table)?;
    }

    globals.set("xcake_monitors", monitors)?;

    Ok(())
}
