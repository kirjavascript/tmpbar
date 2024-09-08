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

    drop(globals);

    (lua, signal)
}
