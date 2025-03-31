use eframe::egui;
use egui::Ui;
use crate::config::{Property, Component, to_property, copy_default};
use crate::global::Global;

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    let props = comp.props();

    if let Some(Property::Function(func)) = props.get("render") {
        let mode = global.i3mode.get();
        let result = func.call::<String, mlua::Value>(mode);

        match result {
            Ok(component) => {
                let default_props = copy_default(props);
                if let Property::Component(mut comp) = to_property(component, &default_props) {
                    crate::components::render(&mut comp, ui, global);
                }
            },
            Err(err) => {
                error!("{}", err);
            },
        }
    }
}
