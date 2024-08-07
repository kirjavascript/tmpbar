use eframe::egui;
use egui::Ui;
use egui_extras::{Size, Strip, StripBuilder};
use crate::config::{Property, Props, Component};
use crate::components::core;
use crate::global::Global;

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    let props = comp.props();
    let is_horizontal = if let Some(Property::String(dir)) = props.get("orientation") { dir.starts_with("h") } else { true };
    let is_flex: bool = props.get("flex").unwrap_or_default().into();

    if !is_flex {
        let layout = core::layout_from_props(props);

        if let Some(Property::Array(list)) = props.get_mut("items") {

            ui.with_layout(layout, |ui| {
                for prop in list {
                    if let Property::Component(comp) = prop {
                        crate::components::render(comp, ui, global);
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

    fn render_components(props: &mut Props, mut strip: Strip, global: &mut Global) {
        if let Some(Property::Array(list)) = props.get_mut("items") {
            for prop in list {
                if let Property::Component(comp) = prop {
                    strip.cell(|ui| {

                        core::render_layout(comp, ui, |comp, ui| {
                            crate::components::render(comp, ui, global);
                        });
                    });
                }
            }
        }
    }

    if is_horizontal {
        builder.horizontal(|strip| {
            render_components(props, strip, global);
        });
    } else {
        builder.vertical(|strip| {
            render_components(props, strip, global);
        });
    }
}
