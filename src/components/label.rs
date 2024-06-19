use eframe::egui;
use egui::Ui;
use crate::config::{Props, get_text};

pub fn render(props: &mut Props, ui: &mut Ui) {
    ui.label(get_text(props, "text"));
}
