use eframe::egui;
use egui::Ui;
use crate::config::{Property, Component, to_property, copy_default};
use crate::global::Global;
use crate::wm::xcb::workspaces::Workspace;
use crate::components::core;

use egui_taffy::tui;
use egui_taffy::TuiBuilderLogic;

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    let style = core::style(comp, ui);
    let props = comp.props();

    let monitor_index: i64 = props.get("_monitor_index").unwrap_or_default().into();
    let show_all: bool = props.get("show_all").unwrap_or_default().into();

    let workspaces: Vec<Workspace> = global.workspaces.list().into_iter().filter(|workspace| {
        show_all || workspace.monitor_index == monitor_index as u32
    }).collect();


    if let Some(Property::Function(func)) = props.get("render") {
        tui(ui, ui.id())
            .style(style)
            .show(|tui| {
                for workspace in workspaces {
                    let table = table_from_workspace(&global.lua, &workspace).unwrap();
                    let result = func.call::<mlua::Table, mlua::Value>(table);

                    if result.is_ok() {
                        let component = result.unwrap();

                        let default_props = copy_default(props);
                        if let Property::Component(mut comp) = to_property(component, &default_props) {
                            let ui = tui.egui_ui_mut();
                            let style = core::style(&mut comp, ui);

                            tui
                                .style(style)
                                .ui(|ui| {
                                    crate::components::render(&mut comp, ui, global);
                                });
                        }
                    } else {
                        error!("{}", result.err().unwrap().to_string());
                    }
                }
            });
    }
}

fn table_from_workspace<'a>(lua: &'a mlua::Lua, workspace: &Workspace) -> mlua::Result<mlua::Table<'a>> {
    let table = lua.create_table()?;

    table.set("number", workspace.number)?;
    table.set("name", workspace.name.to_owned())?;
    table.set("focused", workspace.focused)?;
    table.set("urgent", workspace.urgent)?;
    table.set("visible", workspace.visible)?;
    table.set("monitor_index", workspace.monitor_index)?;

    Ok(table)
}
