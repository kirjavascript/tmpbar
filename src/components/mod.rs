use eframe::egui;
use egui::Ui;
use crate::config::Component;

mod button;
mod label;
mod input;
mod container;

pub fn render(comp: &mut Component, ui: &mut Ui) {
    match comp.name() {
        "text-input" => input::render(comp.props(), ui),
        "container" => container::render(comp.props(), ui),
        "button" => button::render(comp.props(), ui),
        "label" => label::render(comp.props(), ui),
        _ => {
            ui.label(format!("[unknown {:?}]", comp.name()));
        },
    }
}
