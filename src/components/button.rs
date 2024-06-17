use eframe::egui;
use egui::{Ui, Response};
use crate::config::{Property, Props, get_text};

pub fn render(props: &Props, ui: &mut Ui) -> Response {
    let response = ui.button(get_text(props, "text"));

    if response.clicked() {
        if let Some(Property::Function(func)) = props.get("onclick") {
            func.call::<(), ()>(()).ok();
        }
    }

    response
}
