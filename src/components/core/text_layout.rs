use crate::config::{Props, Property};

pub fn text_layout(props: &Props) -> Option<egui::Layout> {
    let style_props = match props.get("style") {
        Some(Property::Object(style)) => style,
        _ => return None,
    };

    let text_align = style_props.get("text_align");
    let text_valign = style_props.get("text_valign");

    if text_align.is_none() && text_valign.is_none() {
        return None;
    }

    // valign
    let main_align = if let Some(Property::String(valign)) = text_valign {
        match valign.as_str() {
            "min" | "start" | "top" => egui::Align::Min,
            "max" | "end" | "down" => egui::Align::Max,
            "center" => egui::Align::Center,
            _ => egui::Align::Min,
        }
    } else {
        egui::Align::Min
    };

    // halign
    let cross_align = if let Some(Property::String(align)) = text_align {
        match align.as_str() {
            "min" | "start" | "left" => egui::Align::Min,
            "max" | "end" | "right" => egui::Align::Max,
            "center" => egui::Align::Center,
            _ => egui::Align::Min,
        }
    } else {
        egui::Align::Min
    };

    Some(egui::Layout {
        main_dir: egui::Direction::TopDown,
        main_wrap: false,
        main_align,
        main_justify: true,
        cross_align,
        cross_justify: true,
    })
}
