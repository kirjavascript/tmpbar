use eframe::egui;
use egui::{Color32, Ui};
use egui_extras::{Size, Strip, StripBuilder};
use crate::config::{Property, Props};

pub fn render(props: &mut Props, ui: &mut Ui) {
    let is_horizontal = if let Some(Property::String(dir)) = props.get("direction") { dir.starts_with("h") } else { true };
    let is_flex: bool = props.get("flex").unwrap_or_default().into();

    if !is_flex {
        fn render_components(props: &mut Props, ui: &mut Ui) {
            if let Some(Property::Array(list)) = props.get_mut("items") {
                for prop in list {
                    if let Property::Component(comp) = prop {
                        super::render(comp, ui);
                    }
                }
            }

        }

        if is_horizontal {
            ui.horizontal(|ui| {
                render_components(props, ui);
            });
        } else {
            ui.vertical(|ui| {
                render_components(props, ui);
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
                        ui.painter().rect_filled(
                            ui.available_rect_before_wrap(),
                            0.0,
                            Color32::GREEN,
                        );
                        ui.with_layout(layout_from_props(comp.props()), |ui| {
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

fn layout_from_props(props: &Props) -> egui::Layout {
    let mut layout = egui::Layout::left_to_right(egui::Align::Center);

    if let Some(Property::String(dir)) = props.get("flow") {
        layout.main_dir = match dir.as_str() {
            "left-right" => egui::Direction::LeftToRight,
            "right-left" => egui::Direction::RightToLeft,
            "top-down" => egui::Direction::TopDown,
            "bottom-up" => egui::Direction::BottomUp,
            _ => egui::Direction::LeftToRight,
        };
    }
    if let Some(Property::Boolean(wrap)) = props.get("wrap") {
        layout.main_wrap = *wrap;
    }
    if let Some(Property::String(align)) = props.get("align") {
        layout.main_align = match align.as_str() {
            "min" | "start" => egui::Align::Min,
            "max" | "end" => egui::Align::Max,
            _ => egui::Align::Center
        };
    }
    layout.main_justify = true;
    if let Some(Property::Boolean(justify)) = props.get("justify") {
        layout.main_justify = *justify;
    }
    if let Some(Property::String(align)) = props.get("crossAlign") {
        layout.cross_align = match align.as_str() {
            "min" | "start" => egui::Align::Min,
            "max" | "end" => egui::Align::Max,
            _ => egui::Align::Center
        };
    }
    layout.cross_justify = true;
    if let Some(Property::Boolean(justify)) = props.get("crossJustify") {
        layout.cross_justify = *justify;
    }

    layout
}
