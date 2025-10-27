use egui::{RichText, FontId, FontFamily};
use crate::config::{Props, Property};
use crate::global::Theme;

pub fn richtext(text: impl Into<String>, props: &Props, theme: &Theme) -> egui::RichText {
    let get_family = |family| {
        if family == "monospace" {
            FontFamily::Monospace
        } else if theme.families.contains(family) {
            FontFamily::Name(family.clone().into())
        } else {
            warn!("font {:?} not available. use ui.load_font({0:?}, \"./{0}.ttf\")", family);
            egui::FontFamily::Proportional
        }
    };

    let mut font_family = egui::FontFamily::Proportional;
    let mut font_size = theme.font_size;
    let mut color = theme.color;
    let mut custom_family = false;

    if let Some(Property::Object(style)) = props.get("style") {
        if let Some(Property::String(family)) = style.get("font_family") {
            font_family = get_family(family);
            custom_family = true;
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

    if !custom_family && theme.font_family.is_some() {
        font_family = get_family(theme.font_family.as_ref().unwrap());
    }

    RichText::new(text)
        .font(FontId::new(font_size, font_family))
        .color(color)
}
