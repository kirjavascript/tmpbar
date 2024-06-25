use eframe::egui;
use egui::Ui;
use crate::config::{Component, Property, get_text};

pub fn render(comp: &mut Component, ui: &mut Ui) {
    let props = comp.props();
    let response = ui.button(get_text(props, "text"));

    if response.clicked() {
        if let Some(Property::Function(func)) = props.get("click") {
            func.call::<(), ()>(()).ok();
        }
    }
}
