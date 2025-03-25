use eframe::egui;
use crate::config::Component;
use crate::global::Global;
use egui::{Ui, Vec2, Color32};

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    let (w, h) = global.tray.dimensions;
    let scale = ui.ctx().pixels_per_point();

    let size = Vec2::new((w as f32 / scale) as _, (h as f32 / scale) as _);

    // allocate size
    let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::empty());

    if comp.props().get("showDebug").unwrap_or_default().into() {
        ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(0, 128, 0));
    }

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

    global.tray.set_pos(x_pos as _, y_pos as _);
}
