use eframe::egui;
use egui::Ui;
use crate::config::{Component, Property, get_text};
use crate::global::Global;

pub fn render_background(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    if let Some(Property::Object(style)) = comp.props().get("style") {
        if let Some(Property::Function(func)) = style.get("background") {
            let rect = ui.available_rect_before_wrap();
            super::svg_image(&global.lua, func, &rect).paint_at(ui, rect);
        } else if let Some(Property::String(string)) = style.get("background") {
            background_color(string, ui);
        } else {
            let string = get_text(style, "background_color");

            if string.len() > 0 {
                background_color(&string, ui);
            }
        }

    }
}

fn background_color(text: &str, ui: &mut Ui) {
    match csscolorparser::parse(text) {
        Ok(color) => {
            let [r, g, b, a] = color.to_rgba8();
            ui.painter().rect_filled(ui.available_rect_before_wrap(), 0.0, egui::Color32::from_rgba_unmultiplied(r, g, b, a));
        },
        Err(err) => {
            error!("{}: color {}", text, err);
        },
    }
}
