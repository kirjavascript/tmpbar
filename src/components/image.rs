use eframe::egui;
use egui::Ui;
use crate::config::Component;

pub fn render(comp: &mut Component, ui: &mut Ui) {
    let props = comp.props();
    let parent: String = props.get("_parent_path").unwrap_or_default().into();
    let path: String = props.get("path").unwrap_or_default().into();

    let path = if path.starts_with("file://")
        || path.starts_with("http://")
        || path.starts_with("https://")
        || path.starts_with("/") {
            path
    } else {
        format!("file://{}{}", parent, path)
    };

    ui.add(egui::Image::from_uri(path));
}
