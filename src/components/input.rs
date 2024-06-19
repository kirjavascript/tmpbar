use eframe::egui;
use egui::Ui;
use crate::config::{Property, Props, text_mut};

pub fn render(props: &mut Props, ui: &mut Ui) {
    let text = text_mut(props, "text");

    let response = ui.add(egui::TextEdit::singleline(text));

    let text = text.to_owned();

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
