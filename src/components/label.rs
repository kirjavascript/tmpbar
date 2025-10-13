use eframe::egui;
use egui::{Ui, RichText, FontId, FontFamily};
use crate::config::{Component, get_text, Property};

pub fn render(comp: &mut Component, ui: &mut Ui) {
    let props = comp.props();
    let text = get_text(props, "text");

    let text = if text.len() == 0 {
        " ".to_string()
    } else {
        text
    };

    // TODO: try widget_visual
    // TODO: validate font first
    // TODO: preprocess font to cascade?
    // TODO: function, only apply if style
    // TODO: maybe use family_and_size instead
    // TODO: or just apply the top level everywhere
    // let mut rich_text = RichText::new(text);

    // if let Some(Property::Object(style)) = props.get("style") {
    //     let mut font_family = FontFamily::Proportional;
    //     let mut font_size = 14.0;

    //     if let Some(Property::String(family)) = style.get("font_family") {
    //         font_family = if family == "monospace" {
    //             FontFamily::Monospace
    //         } else {
    //             FontFamily::Name(family.clone().into())
    //         };
    //     }

    //     if let Some(Property::Float(size)) = style.get("font_size") {
    //         font_size = *size as f32;
    //     } else if let Some(Property::Integer(size)) = style.get("font_size") {
    //         font_size = *size as f32;
    //     }

    //     rich_text = rich_text.font(FontId::new(font_size, font_family));

    //     if let Some(Property::String(color)) = style.get("color") {
    //         match csscolorparser::parse(color) {
    //             Ok(parsed_color) => {
    //                 let [r, g, b, a] = parsed_color.to_rgba8();
    //                 rich_text = rich_text.color(egui::Color32::from_rgba_unmultiplied(r, g, b, a));
    //             },
    //             Err(_) => {}
    //         }
    //     }
    // }

    ui.vertical_centered(|ui| {
        ui.label(text);
    });
}
