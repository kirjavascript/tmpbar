use eframe::egui;
use egui::Ui;
use crate::config::{Component, Property, get_text};
use crate::global::Global;

// TODO: https://docs.rs/egui_taffy/0.9.0/egui_taffy/bg/simple/struct.TuiBackground.html

pub fn render_background(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    if let Some(Property::Object(style)) = comp.props().get("style") {
        if let Some(Property::Function(func)) = style.get("background") {
            let rect = ui.available_rect_before_wrap();
            super::svg_image(&global.lua, func, &rect).paint_at(ui, rect);
        } else if let Some(Property::String(string)) = style.get("background") {
            background_color(string, ui);
        } else if let Some(Property::String(string)) = style.get("background_image") {
            let rect = ui.available_rect_before_wrap();
            egui::Image::from_uri(global.resolve_path(string)).paint_at(ui, rect);
        } else {
            let string = get_text(style, "background_color");

            if string.len() > 0 {
                background_color(&string, ui);
            }
        }
    }
}

fn background_color(text: &str, ui: &mut Ui) {
    match crate::util::color_parse(text) {
        Ok(color) => {
            ui.painter().rect_filled(ui.available_rect_before_wrap(), 0.0, color);
        },
        Err(err) => {
            error!("{}", err);
        },
    }
}
