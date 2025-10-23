use crate::config::{Component, Property, Props};
use egui_taffy::taffy;
use taffy::{
    geometry,
    prelude::*,
    style::{
        AlignContent, AlignItems, BoxSizing, Dimension, Display, FlexDirection,
        FlexWrap, LengthPercentageAuto, Overflow, Position, Style, TextAlign,
    },
    Point,
};

pub fn style(comp: &mut Component, ui: &mut egui::Ui) -> Style {
    style_prop("style", comp, ui)
}

pub fn style_prop(prop: &str, comp: &mut Component, ui: &mut egui::Ui) -> Style {
    if let Some(Property::Object(style)) = comp.props().get(prop) {
        style_from_props(style, ui)
    } else {
        Default::default()
    }
}

fn style_from_props(props: &Props, ui: &mut egui::Ui) -> Style {
    let available_size = ui.available_size();
    let ppp = ui.pixels_per_point();

    let mut style: Style = Default::default();

    /*
     * Unimplemented:
     *
     * item_is_table
     * item_is_replaced
     * grid
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
            "content_box" => BoxSizing::ContentBox,
            "border_box" => BoxSizing::BorderBox,
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
        style.scrollbar_width = *width as f32 / ppp;
    }

    if let Some(Property::String(pos)) = props.get("position") {
        style.position = match pos.as_str() {
            "relative" => Position::Relative,
            "absolute" => Position::Absolute,
            _ => Position::default(),
        };
    }

    if let Some(top) = get_lengthauto_from_prop(props, "top", ppp) {
        style.inset.top = top;
    }

    if let Some(right) = get_lengthauto_from_prop(props, "right", ppp) {
        style.inset.right = right;
    }

    if let Some(bottom) = get_lengthauto_from_prop(props, "bottom", ppp) {
        style.inset.bottom = bottom;
    }

    if let Some(left) = get_lengthauto_from_prop(props, "left", ppp) {
        style.inset.left = left;
    }

    let width: Option<&Property> = props.get("width");
    let height: Option<&Property> = props.get("height");
    let size: Option<&Property> = props.get("size");

    if width.is_some() || height.is_some() {
        let mut width = width.map(|s| Into::<String>::into(s));
        let mut height = height.map(|s| Into::<String>::into(s));

        if matches!(width.as_deref(), Some("max")) {
            width = width.map(|_| (available_size.x * ppp).to_string());
        }
        if matches!(height.as_deref(), Some("max")) {
            height = height.map(|_| (available_size.y * ppp).to_string());
        }

        style.size = geometry::Size {
            width: width.map(|s| parse_dimension(&s, ppp)).unwrap_or(Dimension::Auto),
            height: height.map(|s| parse_dimension(&s, ppp)).unwrap_or(Dimension::Auto),
        };
    } else if size.is_some() {
        let size = size.map(|s| Into::<String>::into(s));

        if matches!(size.as_deref(), Some("max")) {
            style.size = geometry::Size {
                width: length(available_size.x),
                height: length(available_size.y),
            };
        } else {
            let size = size.map(|s| parse_dimension(&s, ppp)).unwrap_or(Dimension::Auto);
            style.size = geometry::Size {
                width: size,
                height: size,
            };
        }
    }

    let width: Option<&Property> = props.get("min_width");
    let height: Option<&Property> = props.get("min_height");

    if width.is_some() || height.is_some() {
        let mut width = width.map(|s| Into::<String>::into(s));
        let mut height = height.map(|s| Into::<String>::into(s));

        if matches!(width.as_deref(), Some("max")) {
            width = width.map(|_| (available_size.x * ppp).to_string());
        }
        if matches!(height.as_deref(), Some("max")) {
            height = height.map(|_| (available_size.y * ppp).to_string());
        }

        style.min_size = geometry::Size {
            width: width.map(|s| parse_dimension(&s, ppp)).unwrap_or(Dimension::Auto),
            height: height.map(|s| parse_dimension(&s, ppp)).unwrap_or(Dimension::Auto),
        };
    }

    let width: Option<&Property> = props.get("max_width");
    let height: Option<&Property> = props.get("max_height");

    if width.is_some() || height.is_some() {
        let mut width = width.map(|s| Into::<String>::into(s));
        let mut height = height.map(|s| Into::<String>::into(s));

        if matches!(width.as_deref(), Some("max")) {
            width = width.map(|_| (available_size.x * ppp).to_string());
        }
        if matches!(height.as_deref(), Some("max")) {
            height = height.map(|_| (available_size.y * ppp).to_string());
        }

        style.max_size = geometry::Size {
            width: width.map(|s| parse_dimension(&s, ppp)).unwrap_or(Dimension::Auto),
            height: height.map(|s| parse_dimension(&s, ppp)).unwrap_or(Dimension::Auto),
        };
    }

    if let Some(Property::Float(ar)) = props.get("aspect_ratio") {
        style.aspect_ratio = Some(*ar as _);
    }

    if let Some(margin) = get_spacing_from_props(
        props,
        "margin",
        get_lengthauto_from_prop,
        LengthPercentageAuto::Length(0.),
        ppp,
    ) {
        style.margin = margin;
    }

    if let Some(padding) = get_spacing_from_props(
        props,
        "padding",
        get_length_from_prop,
        LengthPercentage::Length(0.),
        ppp,
    ) {
        style.padding = padding;
    }

    if let Some(border) = get_spacing_from_props(
        props,
        "border",
        get_length_from_prop,
        LengthPercentage::Length(0.),
        ppp,
    ) {
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

    if let Some(gap) = get_length_from_prop(props, "gap", ppp) {
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
            "row_reverse" => FlexDirection::RowReverse,
            "column_reverse" => FlexDirection::ColumnReverse,
            _ => FlexDirection::default(),
        };
    }

    if let Some(Property::String(wrap)) = props.get("flex_wrap") {
        style.flex_wrap = match wrap.as_str() {
            "nowrap" => FlexWrap::NoWrap,
            "wrap" => FlexWrap::Wrap,
            "wrap_reverse" => FlexWrap::WrapReverse,
            _ => FlexWrap::default(),
        };
    }

    if let Some(Property::String(text)) = props.get("flex_basis") {
        style.flex_basis = parse_dimension(text.as_str(), ppp);
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

fn parse_dimension(value: &str, ppp: f32) -> Dimension {
    if value.ends_with('%') {
        if let Ok(percent) = value[..value.len() - 1].parse::<f32>() {
            return Dimension::Percent(percent / 100.0);
        }
    } else if value.ends_with("px") {
        if let Ok(pixels) = value[..value.len() - 2].parse::<f32>() {
            return Dimension::Length(pixels / ppp);
        }
    } else if let Ok(pixels) = value.parse::<f32>() {
        return Dimension::Length(pixels / ppp);
    }

    Dimension::Auto
}

fn get_spacing_from_props<T: Clone>(
    props: &Props,
    prefix: &str,
    get_length_from_prop: impl Fn(&Props, &str, f32) -> Option<T>,
    default: T,
    ppp: f32,
) -> Option<taffy::prelude::Rect<T>> {
    let all_key = prefix.to_string();
    let top_key = format!("{}_top", prefix);
    let right_key = format!("{}_right", prefix);
    let bottom_key = format!("{}_bottom", prefix);
    let left_key = format!("{}_left", prefix);

    if let Some(all) = get_length_from_prop(props, &all_key, ppp) {
        return Some(taffy::prelude::Rect {
            top: all.clone(),
            right: all.clone(),
            bottom: all.clone(),
            left: all,
        });
    }

    let top = get_length_from_prop(props, &top_key, ppp);
    let right = get_length_from_prop(props, &right_key, ppp);
    let bottom = get_length_from_prop(props, &bottom_key, ppp);
    let left = get_length_from_prop(props, &left_key, ppp);

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

fn get_lengthauto_from_prop(props: &Props, key: &str, ppp: f32) -> Option<LengthPercentageAuto> {
    match props.get(key) {
        Some(Property::Integer(value)) => Some(LengthPercentageAuto::Length(*value as f32 / ppp)),
        Some(Property::Float(value)) => Some(LengthPercentageAuto::Length(*value as f32 / ppp)),
        Some(Property::String(value)) => {
            if value == "auto" {
                Some(LengthPercentageAuto::Auto)
            } else {
                match parse_dimension(value, ppp) {
                    Dimension::Length(len) => Some(LengthPercentageAuto::Length(len / ppp)),
                    Dimension::Percent(pct) => Some(LengthPercentageAuto::Percent(pct)),
                    _ => None,
                }
            }
        }
        _ => None,
    }
}

fn get_length_from_prop(props: &Props, key: &str, ppp: f32) -> Option<LengthPercentage> {
    match props.get(key) {
        Some(Property::Integer(value)) => Some(LengthPercentage::Length(*value as f32 / ppp)),
        Some(Property::Float(value)) => Some(LengthPercentage::Length(*value as f32 / ppp)),
        Some(Property::String(value)) => match parse_dimension(value, ppp) {
            Dimension::Length(len) => Some(LengthPercentage::Length(len / ppp)),
            Dimension::Percent(pct) => Some(LengthPercentage::Percent(pct)),
            _ => None,
        },
        _ => None,
    }
}
