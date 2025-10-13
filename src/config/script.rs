use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use super::parse::{Bar, parse_bars, Property};
use crate::util::{Signal, color_parse};

pub struct ConfigScript {
    pub path: PathBuf,
    pub bars: Vec<Bar>,
    pub reload_signal: Signal<()>,
    ctx: egui::Context,
}

impl ConfigScript {
    pub fn reload(&mut self, lua: &mlua::Lua) -> Result<(), String> {
        load(self, lua)?;
        self.set_visual();
        Ok(())
    }
}

pub fn init(path: &PathBuf, ctx: egui::Context, lua: &mlua::Lua) -> ConfigScript {
    let mut script = ConfigScript {
        path: path.to_owned(),
        bars: Vec::new(),
        reload_signal: Signal::new(ctx.clone()),
        ctx,
    };


    let load_result = load(&mut script, lua);
    script.set_visual();

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
        let mut file_result = File::open(&script.path).map_err(|x| {
            format!("{}: {}", script.path.display(), x.to_string())
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

    set_monitors(lua, &globals)?;

    let set_state: mlua::Function = globals.get("xcake_reset_state")?;
    set_state.call::<_, ()>(())?;

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

impl ConfigScript {
    pub fn set_visual(&mut self) {
        if let Some(bar) = self.bars.get(0) {
            if let Some(Property::Object(style)) = bar.container.props_ref().get("style") {
                if let Some(Property::String(color)) = style.get("color") {
                    match color_parse(color) {
                        Ok(color) => {
                            let mut visuals = egui::Visuals::default();
                            visuals.override_text_color = Some(color);
                            self.ctx.set_visuals(visuals);
                        },
                        Err(err) => error!("{}", err),
                    }
                }
            }
        }
    }
}
