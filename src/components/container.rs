use eframe::egui;
use egui::Ui;
use egui_extras::{Size, Strip, StripBuilder};
use crate::config::{Property, Props, Component};
use crate::components::core;

pub fn render(comp: &mut Component, ui: &mut Ui) {
    let props = comp.props();
    let is_horizontal = if let Some(Property::String(dir)) = props.get("orientation") { dir.starts_with("h") } else { true };
    let is_flex: bool = props.get("flex").unwrap_or_default().into();

    if !is_flex {
        let layout = core::layout_from_props(props);

        if let Some(Property::Array(list)) = props.get_mut("items") {

            ui.with_layout(layout, |ui| {
                for prop in list {
                    if let Property::Component(comp) = prop {
                        super::render(comp, ui);
                    }
                }
            });
        }

        return;
    }

    let mut builder = StripBuilder::new(ui);

    // get item sizes
    if let Some(Property::Array(list)) = props.get_mut("items") {
        for item in list {
            if let Property::Component(comp) = item {
                let size = match comp.props().get("size") {
                    Some(Property::Integer(int)) => Size::exact(*int as _),
                    Some(Property::Float(float)) => Size::relative(*float as _),
                    _ => Size::remainder(),
                };

                builder = builder.size(size);
            }
        }
    }

    fn render_components(props: &mut Props, mut strip: Strip) {
        if let Some(Property::Array(list)) = props.get_mut("items") {
            for prop in list {
                if let Property::Component(comp) = prop {
                    strip.cell(|ui| {

                        core::render_layout(comp, ui, |comp, ui| {
                            super::render(comp, ui);
                        });
                    });
                }
            }
        }
    }

    if is_horizontal {
        builder.horizontal(|strip| {
            render_components(props, strip);
        });
    } else {
        builder.vertical(|strip| {
            render_components(props, strip);
        });
    }
}
