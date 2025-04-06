use std::collections::HashMap;
use crate::util::format_bytes;

pub fn bind(lua: &mlua::Lua, globals: &mlua::Table) {
    let mut last_result: HashMap<String, (u64, u64)> = HashMap::new();
    let mut last_time = std::time::Instant::now();

    let network_read = crate::util::throttle_cell(move || {
        let bw = probes::network::read();
        match bw {
            Ok(info) => {
                let mut interfaces: HashMap<String, (f64, f64)> = HashMap::new();

                let now = std::time::Instant::now();
                let interval = (now-last_time).as_secs_f64();
                last_time = now;

                for (name, interface) in info.interfaces.iter() {
                    let (rx_last, tx_last) = *last_result.get(&name.to_string()).unwrap_or(&(0, 0));

                    let (rx, tx) = (
                        interface.received,
                        interface.transmitted,
                    );

                    last_result.insert(name.to_string(), (rx, tx));


                    let rx_diff = (rx.max(rx_last) - rx_last) as f64;
                    let tx_diff = (tx.max(tx_last) - tx_last) as f64;

                    let down = if rx_last > 0 { rx_diff } else { 0. } / interval;
                    let up = if tx_last > 0 { tx_diff } else { 0. } / interval;

                    interfaces.insert(name.to_string(), (down, up));
                }

                return interfaces
            },
            Err(err) => {
                error!("{}", err);
                return HashMap::new()
            },
        }
    }, std::time::Duration::from_millis(999));


    let network = lua.create_function(move |lua, ()| {
        let interfaces = network_read.borrow_mut()();
        let table = lua.create_table().unwrap();

        for (name, (down, up)) in interfaces.iter() {
            let interface = lua.create_table().unwrap();
            interface.set("down", format!("{}/s", format_bytes(*down))).unwrap();
            interface.set("up", format!("{}/s", format_bytes(*up))).unwrap();
            table.set(name.to_string(), interface).unwrap();
        }

        Ok(table)
    }).unwrap();

    globals.set("bandwidth", network).unwrap();

}
