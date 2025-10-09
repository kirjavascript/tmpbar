use eframe::egui;
use crate::config::{Component, Property};
use crate::global::Global;
use egui::{Ui, Vec2};

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    if global.tray.is_none() {
        return
    }

    let tray = global.tray.as_mut().unwrap();

    let (w, h) = tray.dimensions;
    let scale = ui.ctx().pixels_per_point();

    let size = Vec2::new((w as f32 / scale) as _, (h as f32 / scale) as _);

    // grab available size before allocating
    let available_height = scale * ui.available_height().min(
        ui.ctx().screen_rect().height()
    ).round();

    // allocate size
    let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::empty());

    // set position of tray
    let local_pos = rect.min;
    let mut x_pos = scale * local_pos.x as f32;
    let mut y_pos = scale * local_pos.y as f32;

    let rect = ui.ctx().input(|i| {
        i.viewport().inner_rect
    });

    if let Some(rect) = rect {
        x_pos += scale * rect.min.x;
        y_pos += scale * rect.min.y;
    }

    tray.set_pos(x_pos as _, y_pos as _);

    if !ui.ctx().requested_repaint_last_pass() {
        tray.set_size(available_height as u32);
    }

    if let Some(Property::Object(style)) = comp.props().get("style") {
        if let Some(Property::String(color)) = style.get("background_color") {
            tray.set_bgcolor(color);
        }
    }
}
