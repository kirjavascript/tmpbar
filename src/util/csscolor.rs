pub fn color_parse(text: &str) -> Result<egui::Color32, String> {
    match csscolorparser::parse(text) {
        Ok(color) => {
            let [r, g, b, a] = color.to_rgba8();
            Ok(egui::Color32::from_rgba_unmultiplied(r, g, b, a))
        },
        Err(err) => {
            Err(format!("{}: color {}", text, err))
        },
    }
}
