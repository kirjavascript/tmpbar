use probes::disk_usage::{self, DiskUsage};
use crate::util::format_bytes_precision;

#[derive(Clone)]
struct Disk {
    pub filesystem: Option<String>,
    pub one_k_blocks: u64,
    pub one_k_blocks_used: u64,
    pub one_k_blocks_free: u64,
    pub used_percentage: u32,
    pub mountpoint: String,
}

impl From<DiskUsage> for Disk {
    fn from(disk: DiskUsage) -> Self {
        Disk {
            filesystem: disk.filesystem,
            one_k_blocks: disk.one_k_blocks,
            one_k_blocks_used: disk.one_k_blocks_used,
            one_k_blocks_free: disk.one_k_blocks_free,
            used_percentage: disk.used_percentage,
            mountpoint: disk.mountpoint,
        }
    }
}

pub fn bind(lua: &mlua::Lua, globals: &mlua::Table) {
    let read = crate::util::throttle_cell(move || {
        let usage = disk_usage::read();

        if let Ok(disks) = usage {
            return Some(disks.into_iter().map(Disk::from).collect::<Vec<Disk>>())
        }

        None
    }, std::time::Duration::from_millis(3000));


    let disk = lua.create_function(move |lua, ()| {
        let data = read.borrow_mut()();

        let table = lua.create_table().unwrap();

        if let Some(disks) = data {
            for disk in disks {
                let disk_table = lua.create_table()?;

                if let Some(fs) = &disk.filesystem {
                    disk_table.set("filesystem", fs.to_string())?;
                } else {
                    disk_table.set("filesystem", "")?;
                }

                disk_table.set("total", format_bytes_precision((disk.one_k_blocks * 1024) as _, Some(0)))?;
                disk_table.set("used", format_bytes_precision((disk.one_k_blocks_used * 1024) as _, Some(0)))?;
                disk_table.set("free", format_bytes_precision((disk.one_k_blocks_free * 1024) as _, Some(0)))?;
                disk_table.set("used_percentage", disk.used_percentage)?;

                table.set(disk.mountpoint.clone(), disk_table)?;
            }
        }

        Ok(table)
    }).unwrap();

    globals.set("xcake_disk", disk).unwrap();
}
