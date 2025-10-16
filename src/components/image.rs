use eframe::egui;
use egui::Ui;
use crate::config::{Component, Property};
use crate::components::core;
use crate::global::Global;

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    // SVG render
    if let Some(Property::Function(func)) = comp.props().get("markup") {
        let rect = ui.available_rect_before_wrap();

        ui.add(core::svg_image(&global.lua, func, &rect));
        return;
    }

    // from file
    let path: String = comp.props().get("path").unwrap_or_default().into();
    let image = egui::Image::from_uri(global.resolve_path(&path));
    let image = image.fit_to_original_size(1.);
    let size = image.load_and_calc_size(ui, egui::Vec2 {
        x: f32::MAX,
        y: f32::MAX,
    });

    if let Some(size) = size {
        let available = ui.available_size();
        let width = available.y / size.y * size.x;

        let mut image = image;

        if available.x > 0. && width > available.x {
            image = image.max_size(egui::Vec2 {
                x: available.x,
                y: available.x / size.x * size.y,
            });
        } else {
            image = image.max_size(egui::Vec2 {
                x: width,
                y: available.y,
            });
        }

        ui.centered_and_justified(|ui| {
            ui.add(image);
        });
    }
}
