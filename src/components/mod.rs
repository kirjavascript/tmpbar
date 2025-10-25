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
mod shader;
mod tray;
mod workspaces;

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    core::render_background(comp, ui, global);

    match comp.name() {
        "input" => input::render(comp, ui),
        "container" => container::render(comp, ui, global),
        "button" => button::render(comp, ui),
        "image" => image::render(comp, ui, global),
        "label" => label::render(comp, ui, global),
        "shader" => shader::render(comp, ui, global),
        "tray" => tray::render(comp, ui, global),
        "workspaces" => workspaces::render(comp, ui, global),
        _ => {
            ui.label(format!("[unknown {:?}]", comp.name()));
        }
    }

    core::apply_scroll(comp, ui);
}
