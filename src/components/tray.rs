use eframe::egui;
use crate::config::Component;
use crate::global::Global;
use egui::{Ui, Vec2, Color32};

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    // let (w, h) = global.tray.dimensions();

    // let local_pos = ui.min_rect().min;
    // let mut x_pos = local_pos.x as f32;
    // let mut y_pos = local_pos.y as f32;

    // let rect = ui.ctx().input(|i| {
    //     i.viewport().outer_rect
    // });

    // if let Some(rect) = rect {
    //     x_pos += rect.min.x;
    //     y_pos += rect.min.y;
    // }

    // println!("{} {}", x_pos, y_pos);


    // let x = 10.;
    // let y = 10.;

    // let size = Vec2::new(x as _, y as _);
    // let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());

    // if ui.is_rect_visible(rect) {
    //     ui.painter().rect_filled(rect, 0.0, Color32::from_rgb(255, 0, 0));
    // }

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
