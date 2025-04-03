use eframe::egui;
use egui::Ui;
use crate::config::{Component, get_text};

pub fn render(comp: &mut Component, ui: &mut Ui) {
    let props = comp.props();
    ui.label(get_text(props, "text"));
}
