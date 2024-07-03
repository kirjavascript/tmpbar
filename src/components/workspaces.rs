use eframe::egui;
use egui::Ui;
use crate::config::{Property, Component};
use crate::global::Global;
use crate::wm::xcb::workspaces::Workspace;

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    let props = comp.props();

    let monitor_index: i64 = props.get("_monitor_index").unwrap_or_default().into();
    let show_all: bool = props.get("showAll").unwrap_or_default().into();

    let workspaces: Vec<Workspace> = global.workspaces.list().into_iter().filter(|workspace| {
        show_all || workspace.monitor_index == monitor_index as u32
    }).collect();

    if let Some(Property::Function(func)) = props.get("render") {
        for workspace in workspaces {


            // func.call::<mlua::Table, String>((workspace));

        }
    }
}
