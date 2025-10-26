use systemstat::{System, Platform};
use std::fs;

#[derive(Clone)]
struct CpuStats {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
}

impl CpuStats {
    fn total(&self) -> u64 {
        self.user + self.nice + self.system + self.idle + self.iowait + self.irq + self.softirq
    }

    fn active(&self) -> u64 {
        self.user + self.nice + self.system + self.irq + self.softirq
    }
}

fn read_cpu_stats() -> Option<CpuStats> {
    let contents = fs::read_to_string("/proc/stat").ok()?;
    let first_line = contents.lines().next()?;

    if !first_line.starts_with("cpu ") {
        return None;
    }

    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 8 {
        return None;
    }

    Some(CpuStats {
        user: parts[1].parse().ok()?,
        nice: parts[2].parse().ok()?,
        system: parts[3].parse().ok()?,
        idle: parts[4].parse().ok()?,
        iowait: parts[5].parse().ok()?,
        irq: parts[6].parse().ok()?,
        softirq: parts[7].parse().ok()?,
    })
}

pub fn bind(lua: &mlua::Lua, globals: &mlua::Table) {
    let sys = System::new();

    let temp_read = crate::util::throttle_cell(move || {
        sys.cpu_temp().ok()
    }, std::time::Duration::from_millis(3000));

    let cpu_temp = lua.create_function(move |_lua, ()| {
        let data: Option<f32> = temp_read.borrow_mut()();
        Ok(data)
    }).unwrap();

    let usage_read = crate::util::throttle_cell({
        let mut prev_stats: Option<CpuStats> = None;

        move || {
            let current_stats = read_cpu_stats()?;

            let usage_percent = if let Some(prev) = prev_stats.as_ref() {
                let total_diff = current_stats.total() - prev.total();
                let active_diff = current_stats.active() - prev.active();

                if total_diff > 0 {
                    (active_diff as f64 / total_diff as f64) * 100.0
                } else {
                    0.0
                }
            } else {
                0.0
            };

            prev_stats = Some(current_stats);
            Some(usage_percent)
        }
    }, std::time::Duration::from_millis(3000));

    let cpu_usage = lua.create_function(move |_lua, ()| {
        let usage_percent: Option<f64> = usage_read.borrow_mut()();
        Ok(usage_percent.unwrap_or(0.0))
    }).unwrap();

    globals.set("xcake_cpu_temp", cpu_temp).unwrap();
    globals.set("xcake_cpu_usage", cpu_usage).unwrap();
}
