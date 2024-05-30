use std::fs::File;
use std::io::prelude::*;
use mlua::Lua;

type Config = ();

pub fn load(path: &str) -> Config {
    let script = load_raw(path);

    match script {
        Ok(script) => script,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(0);
        }
    }
}

pub fn load_raw(path: &str) -> Result<Config, String> {
    let path = std::path::Path::new(path);

    let mut file_result = File::open(path).map_err(|x| {
        format!("{}: {}", path.display(), x.to_string().to_lowercase())
    })?;

    let mut contents = String::new();
    file_result
        .read_to_string(&mut contents)
        .map_err(|x| x.to_string())?;


    eval(contents).map_err(|err| err.to_string())
}

pub fn eval(script: String) -> mlua::Result<()> {
    let lua = Lua::new();
    let globals = lua.globals();
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

    globals.set("monitors", monitors)?;

    lua.load(script).exec()?;

    Ok(())
}
