use eframe::egui;
use egui::{Color32, Ui};
use crate::config::{Property, Props};

pub fn layout_from_props(props: &Props) -> egui::Layout {
    let mut layout = egui::Layout::left_to_right(egui::Align::Center);

    if let Some(Property::String(dir)) = props.get("direction") {
        layout.main_dir = match dir.as_str() {
            "left-right" | ">" => egui::Direction::LeftToRight,
            "right-left" | "<" | "reverse" => egui::Direction::RightToLeft,
            "top-down" | "v" => egui::Direction::TopDown,
            "bottom-up" | "^" => egui::Direction::BottomUp,
            _ => egui::Direction::LeftToRight,
        };
    }
    if let Some(Property::Boolean(wrap)) = props.get("wrap") {
        layout.main_wrap = *wrap;
    }
    if let Some(Property::String(align)) = props.get("align") {
        layout.main_align = match align.as_str() {
            "min" | "start" => egui::Align::Min,
            "max" | "end" => egui::Align::Max,
            "center" => egui::Align::Center,
            _ => egui::Align::Min
        };
    }
    // layout.main_justify = true;
    if let Some(Property::Boolean(justify)) = props.get("justify") {
        layout.main_justify = *justify;
    }
    if let Some(Property::String(align)) = props.get("crossAlign") {
        layout.cross_align = match align.as_str() {
            "min" | "start" => egui::Align::Min,
            "max" | "end" => egui::Align::Max,
            "center" => egui::Align::Center,
            _ => egui::Align::Min
        };
    }
    // layout.cross_justify = true;
    if let Some(Property::Boolean(justify)) = props.get("crossJustify") {
        layout.cross_justify = *justify;
    }

    layout
}

pub fn debug_layout(ui: &mut Ui) {
    ui.painter().rect_filled(
        ui.available_rect_before_wrap(),
        0.0,
        Color32::PLACEHOLDER,
    );
}
