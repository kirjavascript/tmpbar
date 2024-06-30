use eframe::egui;
use egui::Ui;
use crate::config::Component;

mod core;
mod button;
mod container;
mod label;
mod image;
mod input;

pub fn render(comp: &mut Component, ui: &mut Ui) {
    core::render_background(comp, ui);
    core::render_frame(comp, ui, |comp, ui| {
        render_impl(comp, ui);
    });
}

fn render_impl(comp: &mut Component, ui: &mut Ui) {
    match comp.name() {
        "input" => input::render(comp, ui),
        "container" => container::render(comp, ui),
        "button" => button::render(comp, ui),
        "image" => image::render(comp, ui),
        "label" => label::render(comp, ui),
        _ => { ui.label(format!("[unknown {:?}]", comp.name())); },
    }
}
