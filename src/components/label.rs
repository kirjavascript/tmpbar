use eframe::egui;
use egui::{Ui, Response};
use crate::config::{Props, get_text};

pub fn render(props: &Props, ui: &mut Ui) -> Response {
    ui.label(get_text(props, "text"))
}
