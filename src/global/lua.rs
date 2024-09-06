use crate::util::Signal;

#[derive(Clone)]
pub enum WorkspaceDirection {
    Next,
    Prev,
}

#[derive(Clone)]
pub enum LuaCallback {
    CycleWorkspace(WorkspaceDirection, mlua::OwnedTable)
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
    let cycle_workspace = lua.create_function(move |_, table: mlua::Table| {
        let monitor: mlua::Table = table.get("monitor").unwrap();
        let direction: String = table.get("direction").unwrap_or("next".to_string()).into();
        let direction = if direction == "next".to_string() { WorkspaceDirection::Next } else { WorkspaceDirection::Prev };

        fn_signal.send(LuaCallback::CycleWorkspace(direction, monitor.into_owned()));

        Ok(())
    }).unwrap();

    globals.set("cycleWorkspace", cycle_workspace).unwrap();

    drop(globals);

    (lua, signal)
}
