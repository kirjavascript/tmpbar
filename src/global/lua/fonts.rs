
pub fn bind(ctx: &egui::Context, lua: &mlua::Lua, globals: &mlua::Table) {
    let ctx_clone = ctx.clone();

    let load_font = lua.create_function(move |_, (name, path): (String, String)| {
        match std::fs::read(&path) {
            Ok(font_data) => {
                ctx_clone.fonts_mut(|fonts| {


                    let mut font_definitions = fonts.definitions().clone();

                    font_definitions.font_data.insert(name.clone(), egui::FontData::from_owned(font_data).into());

                    font_definitions.families.insert(
                        egui::FontFamily::Name(name.clone().into()),
                        vec![name.clone()]
                    );

                    println!("1{:?}",font_definitions.families.get_mut(&egui::FontFamily::Proportional).unwrap());
                    font_definitions.families.get_mut(&egui::FontFamily::Proportional).unwrap()
                        .insert(0, name.to_owned());

                    println!("2{:?}",font_definitions.families.get_mut(&egui::FontFamily::Proportional).unwrap());

                    ctx_clone.set_fonts(font_definitions);
                });
                Ok(())
            },
            Err(e) => {
                error!("Failed to load font {}: {}", path, e);
                Ok(())
            },
        }
    }).unwrap();

    globals.set("xcake_load_font", load_font).unwrap();


    //     let network = lua.create_function(move |lua, ()| {
    //         let interfaces = network_read.borrow_mut()();
    //         let table = lua.create_table().unwrap();

    //         for (name, (down, up)) in interfaces.iter() {
    //             let interface = lua.create_table().unwrap();
    //             interface.set("down", format!("{}/s", format_bytes(*down))).unwrap();
    //             interface.set("up", format!("{}/s", format_bytes(*up))).unwrap();
    //             table.set(name.to_string(), interface).unwrap();
    //         }

    //         Ok(table)
    //     }).unwrap();

    //     globals.set("xcake_bandwidth", network).unwrap();

}
