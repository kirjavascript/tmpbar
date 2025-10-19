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

    // let image = image.fit_to_original_size(1.);
    // let size = image.load_and_calc_size(ui, egui::Vec2 {
    //     x: 100.,
    //     y: 100.,
    // });

    // println!("{size:#?}");

    // if let Some(size) = size {
    //     let available = ui.available_size();

    //     // use paint at / reserve space
    //     let width = available.y / size.y * size.x;

    //     // let mut image = image.fit_to_original_size(2.);

    //     // TODO: center

    //     if available.x > 0. && width > available.x {
    //         let width = available.x;
    //         let height = available.x / size.x * size.y;
    //         let size = egui::Pos2 { x: width, y: height };

    //         let min = ui.next_widget_position();

    //         ui.allocate_space(size.to_vec2());

    //         image.paint_at(ui, egui::Rect {
    //             min,
    //             max: size,
    //         });
    //     } else {
    //         let height = available.y;
    //         let size = egui::Pos2 { x: width, y: height };

    //         let min = ui.next_widget_position();

    //         ui.allocate_space(size.to_vec2());

    //         image.paint_at(ui, egui::Rect {
    //             min,
    //             max: size,
    //         });
    //     }

        ui.centered_and_justified(|ui| {
            ui.add(image);
        });
    // }
}
