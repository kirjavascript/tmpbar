use eframe::egui;
use egui::Ui;
use crate::config::Props;

pub fn render(props: &mut Props, ui: &mut Ui) {
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
