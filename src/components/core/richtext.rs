use egui::{RichText, FontId, FontFamily};
use crate::config::{Props, Property};
use crate::global::Theme;

pub fn richtext(text: impl Into<String>, props: &Props, theme: &Theme) -> egui::RichText {
    let mut font_family = theme.font_family.clone();
    let mut font_size = theme.font_size;
    let mut color = theme.color;

    if let Some(Property::Object(style)) = props.get("style") {
        if let Some(Property::String(family)) = style.get("font_family") {
            if family == "monospace" {
                font_family = FontFamily::Monospace;
            } else {
                font_family = FontFamily::Name(family.clone().into());
            }
        }

        if let Some(Property::Float(size)) = style.get("font_size") {
            font_size = *size as f32;
        } else if let Some(Property::Integer(size)) = style.get("font_size") {
            font_size = *size as f32;
        }

        if let Some(Property::String(color_str)) = style.get("color") {
            match crate::util::color_parse(color_str) {
                Ok(parsed_color) => {
                    color = parsed_color;
                },
                Err(_) => {}
            }
        }
    }

    RichText::new(text)
        .font(FontId::new(font_size, font_family))
        .color(color)
}
