use eframe::egui;
use egui::Ui;
use crate::config::{Component, get_text};
use crate::global::Global;
use crate::components::core;

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    let props = comp.props();
    let text = get_text(props, "text");

    let text = if text.len() == 0 {
        " ".to_string()
    } else {
        text
    };

    let rich_text = core::richtext(text, props, &global.theme);

    ui.vertical_centered(|ui| {
        ui.label(rich_text);
    });
}
