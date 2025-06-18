use eframe::egui;
use egui::Ui;
use crate::config::{Property, Props, Component};
use crate::components::core;
use crate::global::Global;

use egui_extras::{Size, Strip, StripBuilder};

use egui_taffy::{tui, taffy};
use taffy::{
    style::{
        Style,
        Display,
        Dimension,
        FlexDirection,
        FlexWrap,
        AlignItems,
        AlignContent,
        BoxSizing,
        JustifyContent,
        Overflow,
        Position,
        LengthPercentageAuto,
        TextAlign,
    },
    geometry,
    Point,
    prelude::*,
};
use egui_taffy::TuiBuilderLogic;

// TODO: cache?
//https://docs.rs/egui/latest/egui/struct.Memory.html
pub fn style_from_props(props: &Props) -> Style {
    let mut style = Style {
        // display: Display::Flex,
        // padding: length(8.),
        ..Default::default()
    };

    /*
     * Unimplemented:
     *
     * item_is_table
     * item_is_replaced
     */

    if let Some(Property::String(dir)) = props.get("display") {
        style.display = match dir.as_str() {
            "block" => Display::Block,
            "flex" => Display::Flex,
            "grid" => Display::Grid,
            "none" => Display::None,
            _ => Display::default(),
        };
    }

    if let Some(Property::String(sizing)) = props.get("box_sizing") {
        style.box_sizing = match sizing.as_str() {
            "content-box" => BoxSizing::ContentBox,
            "border-box" => BoxSizing::BorderBox,
            _ => BoxSizing::default(),
        };
    }

    if let Some(Property::String(overflow)) = props.get("overflow") {
        style.overflow = match overflow.as_str() {
            "visible" => Point {
                x: Overflow::Visible,
                y: Overflow::Visible,
            },
            "hidden" => Point {
                x: Overflow::Hidden,
                y: Overflow::Hidden,
            },
            "scroll" => Point {
                x: Overflow::Scroll,
                y: Overflow::Scroll,
            },
            "clip" => Point {
                x: Overflow::Clip,
                y: Overflow::Clip,
            },
            _ => Point {
                x: Overflow::Visible,
                y: Overflow::Visible,
            },
        };
    }

    if let Some(Property::String(overflow_x)) = props.get("overflow_x") {
        style.overflow.x = match overflow_x.as_str() {
            "visible" => Overflow::Visible,
            "hidden" => Overflow::Hidden,
            "scroll" => Overflow::Scroll,
            "clip" => Overflow::Clip,
            _ => Overflow::Visible,
        };
    }

    if let Some(Property::String(overflow_y)) = props.get("overflow_y") {
        style.overflow.y = match overflow_y.as_str() {
            "visible" => Overflow::Visible,
            "hidden" => Overflow::Hidden,
            "scroll" => Overflow::Scroll,
            "clip" => Overflow::Clip,
            _ => Overflow::Visible,
        };
    }

    if let Some(Property::Float(width)) = props.get("scrollbar_width") {
        style.scrollbar_width = *width as _;
    }

    if let Some(Property::String(pos)) = props.get("position") {
        style.position = match pos.as_str() {
            "relative" => Position::Relative,
            "absolute" => Position::Absolute,
            _ => Position::default(),
        };
    }

    if let Some(top) = get_lengthauto_from_prop(props, "top") {
        style.inset.top = top;
    }

    if let Some(right) = get_lengthauto_from_prop(props, "right") {
        style.inset.right = right;
    }

    if let Some(bottom) = get_lengthauto_from_prop(props, "bottom") {
        style.inset.bottom = bottom;
    }

    if let Some(left) = get_lengthauto_from_prop(props, "left") {
        style.inset.left = left;
    }

    let width: Option<&Property> = props.get("width");
    let height: Option<&Property> = props.get("height");

    if width.is_some() || height.is_some() {
        let width = width.map(|s| parse_dimension(&Into::<String>::into(s))).unwrap_or(Dimension::Auto);
        let height = height.map(|s| parse_dimension(&Into::<String>::into(s))).unwrap_or(Dimension::Auto);
        style.size = geometry::Size { width, height };
    }

    let width: Option<&Property> = props.get("min_width");
    let height: Option<&Property> = props.get("min_height");

    if width.is_some() || height.is_some() {
        let width = width.map(|s| parse_dimension(&Into::<String>::into(s))).unwrap_or(Dimension::Auto);
        let height = height.map(|s| parse_dimension(&Into::<String>::into(s))).unwrap_or(Dimension::Auto);
        style.min_size = geometry::Size { width, height };
    }

    let width: Option<&Property> = props.get("max_width");
    let height: Option<&Property> = props.get("max_height");

    if width.is_some() || height.is_some() {
        let width = width.map(|s| parse_dimension(&Into::<String>::into(s))).unwrap_or(Dimension::Auto);
        let height = height.map(|s| parse_dimension(&Into::<String>::into(s))).unwrap_or(Dimension::Auto);
        style.max_size = geometry::Size { width, height };
    }

    if let Some(Property::Float(ar)) = props.get("aspect_ratio") {
        style.aspect_ratio = Some(*ar as _);
    }

    if let Some(margin) = get_spacing_from_props(props, "margin", get_lengthauto_from_prop, LengthPercentageAuto::Length(0.)) {
        style.margin = margin;
    }

    if let Some(padding) = get_spacing_from_props(props, "padding", get_length_from_prop, LengthPercentage::Length(0.)) {
        style.padding = padding;
    }

    if let Some(border) = get_spacing_from_props(props, "border", get_length_from_prop, LengthPercentage::Length(0.)) {
        style.border = border;
    }

    if let Some(Property::String(text)) = props.get("align_items") {
        style.align_items = parse_align(text.as_str());
    }

    if let Some(Property::String(text)) = props.get("align_self") {
        style.align_self = parse_align(text.as_str());
    }

    if let Some(Property::String(text)) = props.get("justify_items") {
        style.justify_items = parse_align(text.as_str());
    }

    if let Some(Property::String(text)) = props.get("justify_self") {
        style.justify_self = parse_align(text.as_str());
    }

    if let Some(Property::String(text)) = props.get("align_content") {
        style.align_content = parse_content(text.as_str());
    }

    if let Some(Property::String(text)) = props.get("justify_content") {
        style.justify_content = parse_content(text.as_str());
    }

    if let Some(gap) = get_length_from_prop(props, "gap") {
        style.gap = geometry::Size {
            width: gap,
            height: gap,
        };
    }

    if let Some(Property::String(text)) = props.get("text_align") {
        style.text_align = match text.as_str() {
            "auto" => TextAlign::Auto,
            "left" => TextAlign::LegacyLeft,
            "right" => TextAlign::LegacyRight,
            "center" => TextAlign::LegacyCenter,
            _ => TextAlign::default(),
        };
    }

    if let Some(Property::String(direction)) = props.get("flex_direction") {
        style.flex_direction = match direction.as_str() {
            "row" => FlexDirection::Row,
            "column" => FlexDirection::Column,
            "row-reverse" => FlexDirection::RowReverse,
            "column-reverse" => FlexDirection::ColumnReverse,
            _ => FlexDirection::default(),
        };
    }

    if let Some(Property::String(wrap)) = props.get("flex_wrap") {
        style.flex_wrap = match wrap.as_str() {
            "nowrap" => FlexWrap::NoWrap,
            "wrap" => FlexWrap::Wrap,
            "wrap-reverse" => FlexWrap::WrapReverse,
            _ => FlexWrap::default(),
        };
    }

    if let Some(Property::String(text)) = props.get("flex_basis") {
        style.flex_basis = parse_dimension(text.as_str());
    }


    if let Some(Property::Float(num)) = props.get("flex_grow") {
        style.flex_grow = *num as _;
    }

    if let Some(Property::Float(num)) = props.get("flex_shrink") {
        style.flex_shrink = *num as _;
    }


    style
}

fn parse_content(text: &str) -> Option<AlignContent> {
    match text {
        "start" => Some(AlignContent::Start),
        "end" => Some(AlignContent::End),
        "flex_start" => Some(AlignContent::FlexStart),
        "flex_end" => Some(AlignContent::FlexEnd),
        "center" => Some(AlignContent::Center),
        "stretch" => Some(AlignContent::Stretch),
        "space_between" => Some(AlignContent::SpaceBetween),
        "space_evenly" => Some(AlignContent::SpaceEvenly),
        "space_around" => Some(AlignContent::SpaceAround),
        _ => None,
    }

fn parse_align(text: &str) -> Option<AlignItems> {
    match text {
        "start" => Some(AlignItems::Start),
        "end" => Some(AlignItems::End),
        "flex_start" => Some(AlignItems::FlexStart),
        "flex_end" => Some(AlignItems::FlexEnd),
        "center" => Some(AlignItems::Center),
        "baseline" => Some(AlignItems::Baseline),
        "stretch" => Some(AlignItems::Stretch),
        _ => None,
    }
}

fn parse_dimension(value: &str) -> Dimension {
    if value.ends_with('%') {
        if let Ok(percent) = value[..value.len() - 1].parse::<f32>() {
            return Dimension::Percent(percent / 100.0);
        }
    } else if value.ends_with("px") {
        if let Ok(pixels) = value[..value.len() - 2].parse::<f32>() {
            return Dimension::Length(pixels);
        }
    } else if let Ok(pixels) = value.parse::<f32>() {
        return Dimension::Length(pixels);
    }

    Dimension::Auto
}

fn get_spacing_from_props<T: Clone>(
    props: &Props,
    prefix: &str,
    get_length_from_prop: impl Fn(&Props, &str) -> Option<T>,
    default: T,
) -> Option<taffy::prelude::Rect<T>> {
    let all_key = prefix.to_string();
    let top_key = format!("{}_top", prefix);
    let right_key = format!("{}_right", prefix);
    let bottom_key = format!("{}_bottom", prefix);
    let left_key = format!("{}_left", prefix);

    if let Some(all) = get_length_from_prop(props, &all_key) {
        return Some(taffy::prelude::Rect {
            top: all.clone(),
            right: all.clone(),
            bottom: all.clone(),
            left: all,
        });
    }

    let top = get_length_from_prop(props, &top_key);
    let right = get_length_from_prop(props, &right_key);
    let bottom = get_length_from_prop(props, &bottom_key);
    let left = get_length_from_prop(props, &left_key);

    if top.is_some() || right.is_some() || bottom.is_some() || left.is_some() {
        return Some(taffy::prelude::Rect {
            top: top.unwrap_or_else(|| default.clone()),
            right: right.unwrap_or_else(|| default.clone()),
            bottom: bottom.unwrap_or_else(|| default.clone()),
            left: left.unwrap_or(default),
        });
    }

    None
}

fn get_lengthauto_from_prop(props: &Props, key: &str) -> Option<LengthPercentageAuto> {
    if let Some(Property::Integer(value)) = props.get(key) {
        Some(LengthPercentageAuto::Length(*value as f32))
    } else if let Some(Property::Float(value)) = props.get(key) {
        Some(LengthPercentageAuto::Length(*value as f32))
    } else if let Some(Property::String(value)) = props.get(key) {
        if value == "auto" {
            Some(LengthPercentageAuto::Auto)
        } else {
            match parse_dimension(value) {
                Dimension::Length(len) => Some(LengthPercentageAuto::Length(len)),
                Dimension::Percent(pct) => Some(LengthPercentageAuto::Percent(pct)),
                _ => None,
            }
        }
    } else {
        None
    }
}

fn get_length_from_prop(props: &Props, key: &str) -> Option<LengthPercentage> {
    if let Some(Property::Integer(value)) = props.get(key) {
        Some(LengthPercentage::Length(*value as f32))
    } else if let Some(Property::Float(value)) = props.get(key) {
        Some(LengthPercentage::Length(*value as f32))
    } else if let Some(Property::String(value)) = props.get(key) {
        match parse_dimension(value) {
            Dimension::Length(len) => Some(LengthPercentage::Length(len)),
            Dimension::Percent(pct) => Some(LengthPercentage::Percent(pct)),
            _ => None,
        }
    } else {
        None
    }
}

// TODO: borders
// TODO: position absolute title

// TODO: have a style around each item

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    let min_size = taffy::Size {
        width: length(ui.available_size().x),
        height: length(ui.available_size().y),
    };
    let default_style = || Style {
        display: Display::Flex,
        padding: length(8.),
        gap: length(8.),
        min_size,
            // align_self: Some(taffy::AlignItems::End),
        // justify_content: Some(taffy::AlignContent::FlexEnd),
        ..Default::default()
    };



    tui(ui, ui.id())
        // .reserve_available_space()
        // .reserve_width(1800.)
        .style(Style {
            flex_grow: 1.,
            // flex_basis: taffy::Dimension::Length(500.),
            flex_direction: taffy::FlexDirection::Row,
            align_items: Some(taffy::AlignItems::Stretch),
            // align_self: Some(taffy::AlignItems::End),
            // justify_items: Some(taffy::AlignItems::End),
            // justify_self: Some(taffy::AlignItems::End),
            align_content: Some(taffy::AlignContent::Stretch),
            justify_content: Some(taffy::AlignContent::SpaceBetween),
            ..default_style()
        })
        .show(|tui| {
            // Add egui ui as node

            // tui.ui(|ui| {
            //     ui.label("Hello from egui ui!");
            //     ui.button("Egui button");
            // });

            // tui.ui(|ui| {
            //     ui.label("Hello from egui ui!");
            //     ui.button("Egui button");
            // });

            tui.add_with_border(|tui| {
                tui.label("Text with border");
            });
            tui.add_with_border(|tui| {
                tui.label("Text with border");
            });
            tui.add_with_border(|tui| {
                tui.label("Text with border");
            });
            tui.add_with_border(|tui| {
                tui.label("Text with border");
            });

            // // Add egui widgets directly to UI that implements [`TuiWidget`] trait
            // tui.ui_add(egui::Label::new("label"));
            // tui.ui_add(egui::Button::new("button"));
            // // Or use couple of supported helper function
            // tui.separator();
            // tui.label("Text");

            // // // You can add custom style or unique id to every element that is added to the ui
            // // // by calling id, style, mut_style methods on it first using builder pattern

            // // Provide full style
            // tui.style(Style {
            //     align_self: Some(taffy::AlignItems::Center),
            //     ..Default::default()
            // })
            // .label("Centered text");

            // tui.style(default_style())
            //     .mut_style(|style| {
            //         // Modify one field of the style
            //         style.align_self = Some(taffy::AlignItems::End);
            //     })
            //     .label("Right aligned text");

            // // You can add elements with custom background using add_with_ family of methods
            // tui.add_with_border(|tui| {
            //     tui.label("Text with border");
            // });

            // tui.separator();

            // tui.style(Style {
            //     flex_wrap: taffy::FlexWrap::Wrap,
            //     justify_items: Some(taffy::AlignItems::Stretch),
            //     ..default_style()
            // })
            // .add(|tui| {
            //     for word in FLEX_ITEMS {
            //         tui.style(default_style()).add_with_border(|tui| {
            //             tui.label(word);
            //         });
            //     }
            // });
        });


        return;
    // let props = comp.props();
    // let is_horizontal = if let Some(Property::String(dir)) = props.get("orientation") { dir.starts_with("h") } else { true };
    // let is_flex: bool = props.get("flex").unwrap_or_default().into();

    // if !is_flex {
    //     let layout = core::layout_from_props(props);

    //     if let Some(Property::Array(list)) = props.get_mut("items") {

    //         ui.with_layout(layout, |ui| {
    //             for prop in list {
    //                 if let Property::Component(comp) = prop {
    //                     crate::components::render(comp, ui, global);
    //                 }
    //             }
    //         });
    //     }

    //     return;
    // }

    // let mut builder = StripBuilder::new(ui);

    // // get item sizes
    // if let Some(Property::Array(list)) = props.get_mut("items") {
    //     for item in list {
    //         if let Property::Component(comp) = item {
    //             let size = match comp.props().get("size") {
    //                 Some(Property::Integer(int)) => Size::exact(*int as _),
    //                 Some(Property::Float(float)) => Size::relative(*float as _),
    //                 _ => Size::remainder(),
    //             };

    //             builder = builder.size(size);
    //         }
    //     }
    // }

    // fn render_components(props: &mut Props, mut strip: Strip, global: &mut Global) {
    //     if let Some(Property::Array(list)) = props.get_mut("items") {
    //         for prop in list {
    //             if let Property::Component(comp) = prop {
    //                 strip.cell(|ui| {

    //                     core::render_layout(comp, ui, |comp, ui| {
    //                         crate::components::render(comp, ui, global);
    //                     });
    //                 });
    //             }
    //         }
    //     }
    // }

    // if is_horizontal {
    //     builder.horizontal(|strip| {
    //         render_components(props, strip, global);
    //     });
    // } else {
    //     builder.vertical(|strip| {
    //         render_components(props, strip, global);
    //     });
    // }
}
