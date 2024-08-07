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
    let props = comp.props();

    let path: String = props.get("path").unwrap_or_default().into();

    let path = if path.starts_with("file://")
        || path.starts_with("http://")
        || path.starts_with("https://")
        || path.starts_with("/") {
            path
    } else {
        format!("file://{}{}", global.parent_path, path)
    };

    ui.add(egui::Image::from_uri(path));
}
