pub struct Theme {
    pub color: egui::Color32,
    pub font_family: Option<String>,
    pub font_size: f32,
    pub families: Vec<String>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            color: egui::Color32::from_rgb(180, 180, 180),
            font_family: None,
            font_size: 14.0,
            families: vec!["monospace".to_string()],
        }
    }
}
