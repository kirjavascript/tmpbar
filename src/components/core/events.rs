use crate::config::{Component, Property};
use crate::global::Global;
use crate::util::handle_call;
use egui::Ui;

pub fn events_state(ui: &mut Ui, global: &mut Global) {
    let frame_rect = ui.max_rect();
    let is_hovered = ui.rect_contains_pointer(frame_rect);

    let globals = global.lua.globals();

    globals.set("xcake_event_local_mousedown", is_hovered && ui.input(|i| i.pointer.primary_down())).ok();
}

pub fn events(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    if let Some(Property::Function(func)) = comp.props().get("frame") {
        handle_call(func.call::<(), ()>(()));
    }

    if !global.capture_event {
        return
    }

    let frame_rect = ui.max_rect();
    let is_hovered = ui.rect_contains_pointer(frame_rect);

    if is_hovered {
        if let Some(Property::Function(func)) = comp.props().get("click") {
            let clicked = ui.input(|i| i.pointer.primary_clicked());

            if clicked {
                handle_call(func.call::<(), ()>(()));
                global.capture_event = false;
                return
            }
        }

        if let Some(Property::Function(func)) = comp.props().get("scroll") {
            let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);

            if scroll_delta != 0. {
                handle_call(func.call::<f32, ()>(-scroll_delta)); // negate it to match browser behaviour
                global.capture_event = false;
                return
            }
        }
    }
}
