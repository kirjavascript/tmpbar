use battery::{Battery, Manager};

#[derive(Clone)]
struct BatteryInfo {
    state_of_charge: f32,
    state_of_health: f32,
    energy: f32,
    energy_full: f32,
    energy_full_design: f32,
    voltage: f32,
    temperature: Option<f32>,
    cycle_count: Option<u32>,
    vendor: Option<String>,
    model: Option<String>,
    serial_number: Option<String>,
    technology: String,
    state: String,
}

pub fn bind(lua: &mlua::Lua, globals: &mlua::Table) {
    let manager = Manager::new().ok();
    let mut batteries = manager.as_ref().and_then(|m| {
        m.batteries().ok().map(|iter| {
            iter.filter_map(|b| b.ok()).collect::<Vec<Battery>>()
        })
    }).unwrap_or_default();

    let read = crate::util::throttle_cell(move || {
        if let Some(ref manager) = manager {
            for battery in batteries.iter_mut() {
                let _ = manager.refresh(battery);
            }
        }

        batteries.iter().map(|battery| {
            BatteryInfo {
                state_of_charge: battery.state_of_charge().get::<battery::units::ratio::percent>(),
                state_of_health: battery.state_of_health().get::<battery::units::ratio::percent>(),
                energy: battery.energy().get::<battery::units::energy::watt_hour>(),
                energy_full: battery.energy_full().get::<battery::units::energy::watt_hour>(),
                energy_full_design: battery.energy_full_design().get::<battery::units::energy::watt_hour>(),
                voltage: battery.voltage().get::<battery::units::electric_potential::volt>(),
                temperature: battery.temperature().map(|t| t.get::<battery::units::thermodynamic_temperature::degree_celsius>()),
                cycle_count: battery.cycle_count(),
                vendor: battery.vendor().map(|s| s.to_string()),
                model: battery.model().map(|s| s.to_string()),
                serial_number: battery.serial_number().map(|s| s.to_string()),
                technology: format!("{:?}", battery.technology()),
                state: format!("{:?}", battery.state()),
            }
        }).collect::<Vec<_>>()
    }, std::time::Duration::from_millis(5000));

    let battery_info = lua.create_function(move |lua, ()| {
        let data = read.borrow_mut()();

        let batteries_table = lua.create_table().unwrap();
        for (i, battery) in data.iter().enumerate() {
            let battery_table = lua.create_table().unwrap();
            battery_table.set("charge", battery.state_of_charge).unwrap();
            battery_table.set("health", battery.state_of_health).unwrap();
            battery_table.set("energy", battery.energy).unwrap();
            battery_table.set("energy_full", battery.energy_full).unwrap();
            battery_table.set("energy_full_design", battery.energy_full_design).unwrap();
            battery_table.set("voltage", battery.voltage).unwrap();
            battery_table.set("temperature", battery.temperature).unwrap();
            battery_table.set("cycle_count", battery.cycle_count).unwrap();
            battery_table.set("vendor", battery.vendor.clone()).unwrap();
            battery_table.set("model", battery.model.clone()).unwrap();
            battery_table.set("serial_number", battery.serial_number.clone()).unwrap();
            battery_table.set("technology", battery.technology.clone()).unwrap();
            battery_table.set("state", battery.state.clone()).unwrap();

            batteries_table.set(i + 1, battery_table).unwrap();
        }

        Ok(batteries_table)
    }).unwrap();

    globals.set("xcake_battery_info", battery_info).unwrap();
}
