use crate::config::{Component, Property};
use crate::global::Global;
use egui::Ui;

pub fn events(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    if !global.capture_event {
        return
    }

    let frame_rect = ui.max_rect();
    let is_hovered = ui.rect_contains_pointer(frame_rect);

    if is_hovered {
        if let Some(Property::Function(func)) = comp.props().get("click") {
            let clicked = ui.input(|i| i.pointer.any_click());

            if clicked {
                func.call::<(), ()>(()).ok();
                global.capture_event = false;
                return
            }
        }

        if let Some(Property::Function(func)) = comp.props().get("scroll") {
            let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);

            if scroll_delta != 0. {
                func.call::<f32, ()>(-scroll_delta).ok(); // negate it to match browser behaviour
                global.capture_event = false;
                return
            }
        }
    }
}
