use std::fs::File;
use std::io::prelude::*;
use mlua::Lua;

pub struct ConfigScript {
    path: String,
}

impl ConfigScript {
    pub fn reload(&mut self) -> Result<(), String> {
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
    let lua = Lua::new();
    let globals = lua.globals();

    let bars = lua.create_table()?;
    globals.set("xcake_bars", bars)?;

    lua.load(include_str!("./prelude.lua")).exec()?;

    set_monitors(&lua, &globals)?;

    lua.load(script).exec()?;

    let bars: mlua::Table = globals.get("xcake_bars")?;

    // let bar_list = Vec::new();

    // for pair in bars.pairs::<i32, mlua::Table>() {
    //     let (_, value) = pair?;

    //     bar_list.push(value);
    // }

    info!("{:?}", bars.len());

    Ok(ConfigScript {
        path: path.to_string(),
    })
}

fn set_monitors(lua: &Lua, globals: &mlua::Table) -> mlua::Result<()> {
    let monitors = lua.create_table()?;
    let monitor_list = crate::wm::monitor::list().expect("XrandrError");

    for (i, monitor) in monitor_list.iter().enumerate() {
        let monitor_table = lua.create_table()?;

        monitor_table.set("x", monitor.x)?;
        monitor_table.set("y", monitor.y)?;
        monitor_table.set("width", monitor.width)?;
        monitor_table.set("height", monitor.height)?;
        monitor_table.set("name", monitor.name.to_string())?;

        monitors.set(i + 1, monitor_table)?;
    }

    globals.set("xcake_monitors", monitors)?;

    Ok(())
}
