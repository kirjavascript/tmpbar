use eframe::egui;
use egui::Ui;
use crate::config::{Component, Property};

pub fn render_background(comp: &mut Component, ui: &mut Ui) {
    if let Some(Property::Function(func)) = comp.props().get("background") {
        let rect = ui.available_rect_before_wrap();
        svg_image(func, &rect).paint_at(ui, rect);
    }
}

pub fn svg_image<'a>(func: &mlua::OwnedFunction, rect: &egui::Rect) -> egui::Image<'a> {
    let width = (rect.max.x - rect.min.x).floor();
    let height = (rect.max.y - rect.min.y).floor();

    let markup = func.call::<_, String>((width, height)).unwrap();

    let svg = format!(r#"<?xml version="1.0" standalone="no"?>
        <svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}">
            {markup}
        </svg>
    "#);

    let filename = format!("bytes://{}.svg", fnv1a_hash(&svg));
    let bytes = svg.into_bytes();

    egui::Image::from_bytes(filename, bytes)
}

fn fnv1a_hash(input: &str) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET_BASIS;
    for byte in input.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}
