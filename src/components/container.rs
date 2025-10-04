use eframe::egui;
use egui::Ui;
use crate::config::{Property, Component};
use crate::components::core;
use crate::global::Global;

use egui_taffy::tui;
use egui_taffy::TuiBuilderLogic;
use egui_taffy::taffy::{
    Size,
    Style,
    prelude::*,
};

fn style_from_ui(ui: &mut egui::Ui) -> Style {
    let rect = ui.available_size();

    Style {
        size: Size {
            width: length(rect.x),
            height: length(rect.y),
        },
        ..Default::default()
    }
}

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    let style = core::style_from_component(comp, style_from_ui(ui));
    let props = comp.props();

    tui(ui, ui.id())
        .style(style)
        .show(|tui| {
            if let Some(Property::Array(list)) = props.get_mut("items") {
                for prop in list {
                    if let Property::Component(comp) = prop {
                        let ui = tui.egui_ui_mut();
                        let style = core::style_from_component(comp, style_from_ui(ui));

                        tui
                            .style(style)
                            .ui(|ui| {
                                crate::components::render(comp, ui, global);
                            });
                    }
                }
            }
        });
}
