use eframe::egui;
use egui::Ui;
use crate::config::{Property, Component, get_text};

pub fn render(comp: &mut Component, ui: &mut Ui) {
    let props = comp.props();
    ui.label(get_text(props, "text"));

    if let Some(Property::Integer(interval)) = props.get("interval") {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(*interval as _));
    }
}
