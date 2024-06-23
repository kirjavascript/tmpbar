use eframe::egui;
use egui::Ui;
use crate::config::{Property, Props, get_text};

pub fn render(props: &mut Props, ui: &mut Ui) {
    ui.add(egui::Image::from_uri("file://home/cake/images/animation.gif"));

    // println!("{props:?}");
}
