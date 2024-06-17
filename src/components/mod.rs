use eframe::egui;
use egui::{Ui, Response};
use crate::config::Component;

mod button;
mod label;

pub fn render(comp: &Component, ui: &mut Ui) -> Response {
    match comp.name() {
        "button" => button::render(comp.props(), ui),
        "label" => label::render(comp.props(), ui),
        _ => ui.label(format!("[unknown {:?}]", comp.name())),
    }
}
