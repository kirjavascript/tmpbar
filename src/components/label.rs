use eframe::egui;
use egui::Ui;
use crate::config::{Component, get_text};

pub fn render(comp: &mut Component, ui: &mut Ui) {
    let props = comp.props();
    let text = get_text(props, "text");

    let text = if text.len() == 0 {
        // workaround for position absolute
        " ".to_string()
    } else {
        text
    };

    ui.label(text);
}
