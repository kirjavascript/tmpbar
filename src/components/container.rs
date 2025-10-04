use eframe::egui;
use egui::Ui;
use crate::config::{Property, Component};
use crate::components::core;
use crate::global::Global;

use egui_taffy::tui;
use egui_taffy::TuiBuilderLogic;

fn is_container(name: &str) -> bool {
    name == "container" || name == "workspaces"
}

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    let style = core::style(comp, ui);
    let props = comp.props();

    tui(ui, ui.id())
        .style(style)
        .show(|tui| {
            if let Some(Property::Array(list)) = props.get_mut("items") {
                for prop in list {
                    if let Property::Component(comp) = prop {
                        let ui = tui.egui_ui_mut();

                        let style = if is_container(comp.name()) {
                            core::style_from_ui(ui)
                        } else {
                            core::style(comp, ui)
                        };

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
