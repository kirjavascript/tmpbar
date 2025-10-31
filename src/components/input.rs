use eframe::egui;
use egui::Ui;
use crate::config::{Component, Property, text_mut};
use crate::util::handle_call;

// TODO: https://docs.rs/egui/latest/egui/viewport/enum.ViewportCommand.html

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
            handle_call(func.call::<String, ()>(text.to_owned()));
        }
    }
    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        if let Some(Property::Function(func)) = props.get("submit") {
            handle_call(func.call::<String, ()>(text.to_owned()));
        }
    }
}
