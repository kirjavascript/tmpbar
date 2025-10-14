pub struct Theme {
    pub color: Option<egui::Color32>,
    pub font_family: Option<String>,
    pub font_size: Option<f32>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            color: None,
            font_family: None,
            font_size: None,
        }
    }
}
