use std::fs::{File, canonicalize};
use std::path::Path;
use std::io::prelude::*;
use super::parse::{Bar, parse_bars};
use crate::util::Signal;

pub struct ConfigScript {
    pub path: String,
    pub bars: Vec<Bar>,
    pub reload_signal: Signal<()>,
    lua: mlua::Lua,
}

impl ConfigScript {
    pub fn reload(&mut self) -> Result<(), String> {
        load(self)?;
        Ok(())
    }
}

pub fn init(path: &str, ctx: egui::Context) -> ConfigScript {
    let mut script = ConfigScript {
        path: path.to_owned(),
        bars: Vec::new(),
        reload_signal: Signal::new(ctx),
        lua: mlua::Lua::new(),
    };

    script.lua.load(include_str!("./prelude.lua")).exec().unwrap();

    let load_result = load(&mut script);

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

fn load(script: &mut ConfigScript) -> Result<(), String> {
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

    eval(script, code).map_err(|err| err.to_string())
}

fn eval(script: &mut ConfigScript, code: String) -> mlua::Result<()> {
    let globals = script.lua.globals();

    if let Ok(path) = canonicalize(Path::new(&script.path)) {
        let parent = path.parent().map(|p| p.to_path_buf());
        globals.set("xcake_parent_path", parent.unwrap().to_string_lossy() + "/")?;
    }

    set_monitors(&script.lua, &globals)?;

    let set_state: mlua::Function = script.lua.globals().get("xcake_reset_state")?;
    set_state.call(())?;

    script.lua.load(code).exec()?;

    script.bars = parse_bars(&script.lua)?;

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
