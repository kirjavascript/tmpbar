use eframe::egui;
use egui::Ui;
use crate::config::{Component, get_text};

// https://github.com/emilk/egui/discussions/3868

pub fn render(comp: &mut Component, ui: &mut Ui) {
    let props = comp.props();
    let text = get_text(props, "text");

    let text = if text.len() == 0 {
        // workaround for position absolute
        " ".to_string()
    } else {
        text
    };

    ui.vertical_centered(|ui| {
        ui.label(text);
    });
}
