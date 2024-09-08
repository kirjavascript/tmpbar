use egui::Ui;
use crate::config::{Component, Property};

pub fn apply_scroll(comp: &mut Component, ui: &mut Ui) {
    if let Some(Property::Function(func)) = comp.props().get("scroll") {
        let frame_rect = ui.max_rect();
        let is_hovered = ui.rect_contains_pointer(frame_rect);

        if is_hovered {
            let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);

            if scroll_delta != 0. {
                func.call::<f32, ()>(-scroll_delta).ok(); // negate it to match browser behaviour
            }
        }
    }
}
