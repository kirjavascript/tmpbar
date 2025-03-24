use eframe::egui;
use crate::config::Component;
use crate::global::Global;
use egui::{Ui, Vec2, Color32};

pub fn render(_comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    let (w, h) = global.tray.dimensions;

    let local_pos = ui.min_rect().min;
    let mut x_pos = local_pos.x as f32;
    let mut y_pos = local_pos.y as f32;

    let rect = ui.ctx().input(|i| {
        i.viewport().outer_rect
    });

    if let Some(rect) = rect {
        x_pos += rect.min.x;
        y_pos += rect.min.y;
    }

    y_pos += 20.;

    global.tray.set_pos(x_pos as _, y_pos as _);

    let size = Vec2::new(w as _, h as _);
    let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::hover());

    if ui.is_rect_visible(rect) {
        ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(255, 0, 0));
    }
}
