use systemstat::{System, Platform};

pub fn bind(lua: &mlua::Lua, globals: &mlua::Table) {
    let sys = System::new();

    let read = crate::util::throttle_cell(move || {
        sys.cpu_temp().ok()
    }, std::time::Duration::from_millis(3000));

    let cpu_temp = lua.create_function(move |_lua, ()| {
        let data: Option<f32> = read.borrow_mut()();

        Ok(data)
    }).unwrap();

    globals.set("xcake_cpu_temp", cpu_temp).unwrap();
}
