use eframe::egui;
use egui::Ui;
use crate::config::{Component, Property, text_mut};

pub fn render(comp: &mut Component, ui: &mut Ui) {
    let props = comp.props();
    let text = text_mut(props, "text");

    let response = ui.add(egui::TextEdit::singleline(text));

    let text = text.to_owned();

    if response.gained_focus() || response.clicked() {
        crate::wm::xcb::window_focus(props.get("_bar_id").unwrap_or_default().into(), true);
    }

    if response.lost_focus() {
        crate::wm::xcb::window_focus(props.get("_bar_id").unwrap_or_default().into(), false);
    }


    if response.changed() {
        if let Some(Property::Function(func)) = props.get("change") {
            func.call::<String, ()>(text.to_owned()).ok();
        }
    }
    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        if let Some(Property::Function(func)) = props.get("submit") {
            func.call::<String, ()>(text.to_owned()).ok();
        }
    }
}
