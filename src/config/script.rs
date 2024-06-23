use std::fs::{File, canonicalize};
use std::path::Path;
use std::io::prelude::*;
use super::parse::{ConfigScript, parse_script};

impl ConfigScript {
    pub fn reload(&mut self) -> Result<(), String> {
        info!("reloading config");
        let _ = std::mem::replace(self, load_raw(&self.path)?);
        Ok(())
    }
}

pub fn load(path: &str) -> (impl Fn() -> bool, ConfigScript) {
    let script = load_raw(path);

    match script {
        Ok(script) => {
            let rx = super::watch::init(path);
            let poll = move || rx.try_recv().is_ok();

            (poll, script)
        },
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(0);
        }
    }
}

pub fn load_raw(path: &str) -> Result<ConfigScript, String> {
    let script = {
        let path = std::path::Path::new(path);

        let mut file_result = File::open(path).map_err(|x| {
            format!("{}: {}", path.display(), x.to_string().to_lowercase())
        })?;

        let mut script = String::new();
        file_result
            .read_to_string(&mut script)
            .map_err(|x| x.to_string())?;

        script
    };

    eval(path, script).map_err(|err| err.to_string())
}

pub fn eval(path: &str, script: String) -> mlua::Result<ConfigScript> {
    let lua = mlua::Lua::new();
    let globals = lua.globals();

    if let Ok(path) = canonicalize(Path::new(path)) {
        let parent = path.parent().map(|p| p.to_path_buf());
        globals.set("xcake_parent_path", parent.unwrap().to_string_lossy() + "/")?;
    }

    lua.load(include_str!("./prelude.lua")).exec()?;

    set_monitors(&lua, &globals)?;

    lua.load(script).exec()?;

    parse_script(path, &lua)
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
