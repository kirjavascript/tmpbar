use eframe::egui;
use egui::Ui;
use crate::config::Component;
use crate::global::Global;

pub fn render(_comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    let (x, y) = global.tray.dimensions();

    let pixels: Vec<egui::Color32> = global.tray.framebuffer.chunks_exact(4)
        // .map(|p| egui::Color32::from_rgba_premultiplied(p[2], p[1], p[0], p[3]))
        .map(|p|
            if p[0] == 0 && p[1] == 0 && p[2] == 0 {
                egui::Color32::TRANSPARENT
            } else {
                egui::Color32::from_rgba_premultiplied(p[2], p[1], p[0], p[3])
        })
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

    let response = ui.add(img);

    // ctx.input().pointer.interact_pos()

    if response.clicked() {
        global.tray.click(1, 0);
    }
    if response.secondary_clicked() {
        global.tray.click(3, 0);
    }
    if response.middle_clicked() {
        global.tray.click(2, 0);
    }
}
