use eframe::egui;
use egui::Ui;
use crate::config::Component;

mod button;
mod container;
mod label;
mod image;
mod input;

pub fn render(comp: &mut Component, ui: &mut Ui) {
    if comp.props().get("debugLayout").unwrap_or_default().into() {
        crate::util::debug_layout(ui);
    }

    match comp.name() {
        "input" => input::render(comp, ui),
        "container" => container::render(comp, ui),
        "button" => button::render(comp, ui),
        "image" => image::render(comp, ui),
        "label" => label::render(comp, ui),
        _ => { ui.label(format!("[unknown {:?}]", comp.name())); },
    }
}
