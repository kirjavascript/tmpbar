use eframe::egui;
use egui::Ui;
use crate::config::Component;

mod button;
mod label;
mod input;
mod container;

mod util;

pub fn render(comp: &mut Component, ui: &mut Ui) {
    if comp.props().get("debugLayout").unwrap_or_default().into() {
        util::debug_layout(ui);
    }

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
