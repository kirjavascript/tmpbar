use alsa::mixer::{Mixer, SelemChannelId, SelemId};

#[derive(Clone)]
struct VolumeInfo {
    volume: i64,
    volume_percent: f64,
    is_muted: bool,
    min_volume: i64,
    max_volume: i64,
}

pub fn bind(lua: &mlua::Lua, globals: &mlua::Table) {
    let get_volume_info = || -> Option<VolumeInfo> {
        let mixer = Mixer::new("default", false).ok()?;
        let selem_id = SelemId::new("Master", 0);
        let selem = mixer.find_selem(&selem_id)?;

        let (min, max) = selem.get_playback_volume_range();
        let volume = selem.get_playback_volume(SelemChannelId::mono()).unwrap_or(0);
        let volume_percent = if max > min {
            ((volume - min) as f64 / (max - min) as f64) * 100.0
        } else {
            0.0
        };

        let is_muted = selem.get_playback_switch(SelemChannelId::mono())
            .map(|switch| switch == 0)
            .unwrap_or(false);

        Some(VolumeInfo {
            volume,
            volume_percent,
            is_muted,
            min_volume: min,
            max_volume: max,
        })
    };

    let read = crate::util::throttle_cell(get_volume_info, std::time::Duration::from_millis(50));

    let volume_info = lua.create_function(move |lua, ()| {
        let data = read.borrow_mut()();

        match data {
            Some(info) => {
                let table = lua.create_table().unwrap();
                table.set("volume", info.volume).unwrap();
                table.set("percent", info.volume_percent).unwrap();
                table.set("is_muted", info.is_muted).unwrap();
                table.set("min", info.min_volume).unwrap();
                table.set("max", info.max_volume).unwrap();
                Ok(table)
            }
            None => {
                let table = lua.create_table().unwrap();
                table.set("error", "failed to get volume info").unwrap();
                Ok(table)
            }
        }
    }).unwrap();

    let set_volume = lua.create_function(move |_lua, percent: f64| {
        let mixer = Mixer::new("default", false).ok();
        if let Some(mixer) = mixer {
            let selem_id = SelemId::new("Master", 0);
            if let Some(selem) = mixer.find_selem(&selem_id) {
                let (min, max) = selem.get_playback_volume_range();
                let volume = min + ((percent.clamp(0.0, 100.0) / 100.0) * (max - min) as f64) as i64;
                let _ = selem.set_playback_volume_all(volume);
            }
        }
        Ok(())
    }).unwrap();

    let set_mute = lua.create_function(move |_lua, muted: bool| {
        let mixer = Mixer::new("default", false).ok();
        if let Some(mixer) = mixer {
            let selem_id = SelemId::new("Master", 0);
            if let Some(selem) = mixer.find_selem(&selem_id) {
                let switch_value = if muted { 0 } else { 1 };
                let _ = selem.set_playback_switch_all(switch_value);
            }
        }
        Ok(())
    }).unwrap();

    globals.set("xcake_volume_info", volume_info).unwrap();
    globals.set("xcake_set_volume", set_volume).unwrap();
    globals.set("xcake_set_mute", set_mute).unwrap();
}
