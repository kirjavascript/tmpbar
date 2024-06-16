use eframe::egui;
use egui::{Ui, Response};
use crate::config::Props;

pub fn render(props: &Props, ui: &mut Ui) -> Response {
    let text = match props.get("text") {
        Some(crate::config::Property::Function(func)) => {
            func.call::<(), String>(())
                .unwrap_or("invalid function".to_string())
        }
        Some(crate::config::Property::String(text)) => {
            text.to_owned()
        }
        _ => "invald text".to_string()
    };

    ui.label(text)
}
