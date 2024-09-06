use eframe::egui;
use egui::Ui;
use crate::config::{Component, Property};

pub fn render_frame(comp: &mut Component, ui: &mut Ui, callback: impl FnOnce(&mut Component, &mut Ui)) {
    let props = comp.props();
    let keys = ["margin", "padding"];
    let has_key = keys.iter().any(|&key| props.contains_key(key));

    if !has_key {
        callback(comp, ui);
        return;
    }

    let mut frame = egui::Frame::none();

    if let Some(margin) = comp.props().get("margin") {
        frame.outer_margin = get_margin(margin);
    }
    if let Some(margin) = comp.props().get("padding") {
        frame.inner_margin = get_margin(margin);
    }

    frame.show(ui, |ui| {
        callback(comp, ui);
    });
}

fn get_margin(item: &Property) -> egui::Margin {
    match item {
        Property::Object(obj) => {
            egui::Margin {
                top: obj.get("top").unwrap_or_default().into(),
                bottom: obj.get("bottom").unwrap_or_default().into(),
                left: obj.get("left").unwrap_or_default().into(),
                right: obj.get("right").unwrap_or_default().into(),
            }
        },
        Property::Float(num) => {
            egui::Margin {
                top: *num as _,
                bottom: *num as _,
                left: *num as _,
                right: *num as _,
            }
        },
        Property::Integer(num) => {
            egui::Margin {
                top: *num as _,
                bottom: *num as _,
                left: *num as _,
                right: *num as _,
            }
        },
        _ => egui::Margin::ZERO,
    }
}
