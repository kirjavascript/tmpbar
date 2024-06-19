use eframe::egui;
use egui::Ui;
use crate::config::{Property, Props, get_text};

pub fn render(props: &mut Props, ui: &mut Ui) {
    let response = ui.button(get_text(props, "text"));

    if response.clicked() {
        if let Some(Property::Function(func)) = props.get("click") {
            func.call::<(), ()>(()).ok();
        }
    }
}
