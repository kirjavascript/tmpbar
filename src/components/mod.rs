use eframe::egui;
use egui::{Ui, Response};
use crate::config::Component;

mod label;

pub fn render(comp: &Component, ui: &mut Ui) -> Response {
    match comp.name() {
        "label" => label::render(comp.props(), ui),
        _ => ui.label(format!("[unknown {:?}]", comp.name())),
    }
}
