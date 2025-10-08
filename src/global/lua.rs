use crate::util::Signal;
use crate::wm::xcb::workspaces::WorkspaceDirection;

mod bandwidth;
mod cpu;
mod disk;
mod memory;

#[derive(Clone)]
pub enum LuaCallback {
    CycleWorkspace(WorkspaceDirection),
    FocusWorkspace(u32),
}

pub fn load_lua(ctx: egui::Context) -> (mlua::Lua, Signal<LuaCallback>) {
    let lua = mlua::Lua::new();
    let globals = lua.globals();
    lua.load(include_str!("./lua/prelude.lua")).exec().unwrap();

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
        use std::process::Stdio;
        use std::os::unix::process::CommandExt;

        let child = std::process::Command::new("/bin/sh")
            .arg("-c")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .process_group(0)
            .spawn();

        if let Ok(mut child) = child {
            std::thread::spawn(move || {
                let _ = child.wait();
            });
        }

        Ok(())
    }).unwrap();

    globals.set("xcake_spawn", spawn).unwrap();

    bandwidth::bind(&lua, &globals);
    memory::bind(&lua, &globals);
    cpu::bind(&lua, &globals);
    disk::bind(&lua, &globals);

    drop(globals);

    (lua, signal)
}
