use std::rc::Rc;
use std::cell::RefCell;

pub fn bind(ctx: &egui::Context, lua: &mlua::Lua, globals: &mlua::Table) {
    let ctx_clone = ctx.clone();
    let fonts = Rc::new(RefCell::new(egui::FontDefinitions::default()));

    let load_font = lua.create_function(move |_, (name, path): (String, String)| {
        match std::fs::read(&path) {
            Ok(font_data) => {
                let mut fonts = fonts.borrow_mut();

                fonts.font_data.insert(name.clone(), egui::FontData::from_owned(font_data).into());

                fonts.families.insert(
                    egui::FontFamily::Name(name.clone().into()),
                    vec![name.clone()]
                );

                ctx_clone.set_fonts(fonts.clone());
                Ok(())
            },
            Err(e) => {
                error!("Failed to load font {}: {}", path, e);
                Ok(())
            },
        }
    }).unwrap();

    globals.set("xcake_add_font", load_font).unwrap();
}
