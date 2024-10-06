use crate::util::Signal;
use crate::wm::xcb::workspaces::WorkspaceDirection;

#[derive(Clone)]
pub enum LuaCallback {
    CycleWorkspace(WorkspaceDirection),
    FocusWorkspace(u32),
}

pub fn load_lua(path: &str, ctx: egui::Context) -> (mlua::Lua, Signal<LuaCallback>) {
    let lua = mlua::Lua::new();
    let globals = lua.globals();
    lua.load(include_str!("./prelude.lua")).exec().unwrap();

    // save parent path
    if let Ok(path) = std::fs::canonicalize(std::path::Path::new(path)) {
        let parent = path.parent().map(|p| p.to_path_buf());
        globals.set("xcake_parent_path", parent.unwrap().to_string_lossy() + "/").ok();
    }

    // callbacks

    let signal: Signal<LuaCallback> = Signal::new(ctx);

    let fn_signal = signal.clone();
    let cycle_workspace = lua.create_function(move |_, direction: String| {
        let direction = if direction == "next".to_string() { WorkspaceDirection::Next } else { WorkspaceDirection::Prev };

        fn_signal.send(LuaCallback::CycleWorkspace(direction));

        Ok(())
    }).unwrap();

    globals.set("xcake_cycle_workspace", cycle_workspace).unwrap();

    let fn_signal = signal.clone();
    let focus_workspace = lua.create_function(move |_, desktop: u32| {
        fn_signal.send(LuaCallback::FocusWorkspace(desktop));
        Ok(())
    }).unwrap();

    globals.set("xcake_focus_workspace", focus_workspace).unwrap();

    // API

    let spawn = lua.create_function(move |_, command: String| {
        std::process::Command::new("/bin/sh").arg("-c").arg(command)
            .spawn().ok();
        Ok(())
    }).unwrap();

    globals.set("spawn", spawn).unwrap();

    use std::collections::HashMap;

    let mut last_result: HashMap<String, (u64, u64)> = HashMap::new();
    let mut last_time = std::time::Instant::now();

    let network_read = crate::util::throttle_cell(move|| {
        let bw = probes::network::read();
        match bw {
            Ok(info) => {
                let mut interfaces: HashMap<String, (f64, f64)> = HashMap::new();

                for (name, interface) in info.interfaces.iter() {
                    let (rx_last, tx_last) = *last_result.get(&name.to_string()).unwrap_or(&(0, 0));

                    let (rx, tx) = (
                        interface.received,
                        interface.transmitted,
                    );

                    last_result.insert(name.to_string(), (rx, tx));

                    let now = std::time::Instant::now();
                    let interval = (now-last_time).as_secs_f64();
                    last_time = now;

                    let down = (rx.max(rx_last) - rx_last) as f64 / interval;
                    let up = (tx.max(tx_last) - tx_last) as f64 / interval;

                    interfaces.insert(name.to_string(), (down, up));
                }

                return interfaces
            },
            Err(err) => {
                error!("{}", err);
                return HashMap::new()
            },
        }
    }, std::time::Duration::from_secs(1));


    let network = lua.create_function(move |lua, ()| {
        let interfaces = network_read.borrow_mut()();
        let table = lua.create_table().unwrap();

        for (name, (down, up)) in interfaces.iter() {
            let interface = lua.create_table().unwrap();
            interface.set("down", *down).unwrap();
            interface.set("up", *up).unwrap();
            table.set(name.to_string(), interface).unwrap();
        }

        Ok(table)
    }).unwrap();

    globals.set("bandwidth", network).unwrap();

    drop(globals);

    (lua, signal)
}
