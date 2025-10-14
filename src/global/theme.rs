pub struct Theme {
    pub color: egui::Color32,
    pub font_family: egui::FontFamily,
    pub font_size: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            color: egui::Color32::from_rgb(180, 180, 180),
            font_family: egui::FontFamily::Proportional,
            font_size: 14.0,
        }
    }
}
