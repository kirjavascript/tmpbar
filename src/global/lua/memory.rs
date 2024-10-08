use probes::memory;

#[derive(Clone)]
struct Memory {
    total: u64,
    free: u64,
    used: u64,
    swap_total: u64,
    swap_free: u64,
    swap_used: u64,
    used_percent: String,
    free_percent: String,
}

pub fn bind(lua: &mlua::Lua, globals: &mlua::Table) {
    let read = crate::util::throttle_cell(move || {
        let mem = memory::proc::read().expect("error reading RAM data");

        let total =  mem.total.unwrap_or(0);
        let free =  mem.free.unwrap_or(0);
        let used = mem.used;

        Memory {
            total,
            free,
            used,
            swap_total: mem.swap_total.unwrap_or(0),
            swap_free: mem.swap_free.unwrap_or(0),
            swap_used: mem.swap_used.unwrap_or(0),
            used_percent: format!(
                "{:.2}%",
                (used as f64 / total as f64) * 100.,
            ),
            free_percent: format!(
                "{:.2}%",
                (free as f64 / total as f64) * 100.,
            ),
        }
    }, std::time::Duration::from_millis(450));


    let memory = lua.create_function(move |lua, ()| {
        let data = read.borrow_mut()();

        let table = lua.create_table().unwrap();
        table.set("total", data.total).unwrap();
        table.set("free", data.free).unwrap();
        table.set("used", data.used).unwrap();
        table.set("swap_total", data.swap_total).unwrap();
        table.set("swap_free", data.swap_free).unwrap();
        table.set("swap_used", data.swap_used).unwrap();
        table.set("used_percent", data.used_percent).unwrap();
        table.set("free_percent", data.free_percent).unwrap();

        Ok(table)
    }).unwrap();

    globals.set("memory", memory).unwrap();
}
