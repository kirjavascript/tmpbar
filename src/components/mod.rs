use crate::config::Component;
use crate::global::Global;
use eframe::egui;
use egui::Ui;

mod button;
mod container;
mod core;
mod image;
mod input;
mod label;
mod tray;
mod workspaces;

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    core::render_background(comp, ui, global);
    core::apply_scroll(comp, ui);
    core::render_frame(comp, ui, |comp, ui| {
        render_impl(comp, ui, global);
    });
}

fn render_impl(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    match comp.name() {
        "input" => input::render(comp, ui),
        "container" => container::render(comp, ui, global),
        "button" => button::render(comp, ui),
        "image" => image::render(comp, ui, global),
        "label" => label::render(comp, ui),
        "workspaces" => workspaces::render(comp, ui, global),
        "tray" => tray::render(comp, ui, global),
        _ => {
            ui.label(format!("[unknown {:?}]", comp.name()));
        }
    }
}
