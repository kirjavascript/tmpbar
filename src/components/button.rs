use eframe::egui;
use crate::global::Global;
use egui::Ui;
use crate::config::{Component, Property, get_text};
use crate::util::handle_call;

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    let props = comp.props();
    let response = ui.button(get_text(props, "text"));

    if global.capture_event && response.clicked() {
        if let Some(Property::Function(func)) = props.get("click") {
            handle_call(func.call::<(), ()>(()));
            global.capture_event = false;
        }
    }
}
