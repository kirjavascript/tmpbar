use eframe::egui;
use egui::Ui;
use crate::config::{Property, Component, get_text};
use crate::global::Global;

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    let props = comp.props();

    let (x, y) = global.tray.dimensions();


    let pixels: Vec<egui::Color32> = global.tray.framebuffer.chunks_exact(4)
        .map(|p| egui::Color32::from_rgb(p[2], p[1], p[0]))
        .collect();

    if (x * y) != pixels.len() as _ {
        return;
    }
    let texture: &egui::TextureHandle = &ui.ctx().load_texture(
        "tray",
        egui::ColorImage {
            size: [x as _, y as _],
            pixels,
        },
        egui::TextureOptions::NEAREST
    );

    let img = egui::Image::new(texture)
        .sense(egui::Sense::click());

    ui.add(img);
}
