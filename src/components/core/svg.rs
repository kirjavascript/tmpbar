pub fn svg_image<'a>(lua: &mlua::Lua, func: &mlua::OwnedFunction, rect: &egui::Rect) -> egui::Image<'a> {
    let width = (rect.max.x - rect.min.x).floor();
    let height = (rect.max.y - rect.min.y).floor();

    let table = lua.create_table().unwrap();
    table.set("width", width).unwrap();
    table.set("height", height).unwrap();

    let markup = func.call::<mlua::Table, String>(table);

    let markup = match markup {
        Ok(markup) => markup,
        Err(error) => {
            let error = error.to_string();
            error!("{error}");

            format!("<text fill=\"red\">{error}</text>")
        },
    };

    let svg = format!(r#"<?xml version="1.0" standalone="no"?>
        <svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}">
            {markup}
        </svg>
    "#);

    let filename = format!("bytes://{}.svg", crate::util::fnv1a_hash(&svg));
    let bytes = svg.into_bytes();

    egui::Image::from_bytes(filename, bytes)
}
