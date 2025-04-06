use crate::util::read_file;

pub fn bind(lua: &mlua::Lua, globals: &mlua::Table) {
    let read = crate::util::throttle_cell(move || {
        for i in 0..5 {
            let hwmon_dir = format!("/sys/class/hwmon/hwmon{}", i);

            if let Ok(name) = read_file(&format!("{}/name", hwmon_dir)) {
                let name = name.trim().to_lowercase();

                if name.contains("cpu") || name.contains("coretemp") || name.contains("k10temp") {
                    for j in 1..5 {
                        if let Ok(temp_str) = read_file(&format!("{}/temp{}_input", hwmon_dir, j)) {
                            if let Ok(temp) = temp_str.trim().parse::<f64>() {
                                return Some(format!("{:.1}", temp / 1000.0));
                            }
                        }
                    }
                }
            }
        }

        for i in 0..10 {
            if let Ok(temp_str) = read_file(&format!("/sys/class/thermal/thermal_zone{}/temp", i)) {
                if let Ok(temp) = temp_str.trim().parse::<f64>() {
                    return Some(format!("{:.1}", temp / 1000.0));
                }
            }
        }

        None
    }, std::time::Duration::from_millis(999));


    let cpu_temp = lua.create_function(move |_lua, ()| {
        let data = read.borrow_mut()();

        Ok(data)
    }).unwrap();

    globals.set("cpu_temp", cpu_temp).unwrap();
}
