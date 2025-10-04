use egui_taffy::taffy;
use crate::config::{Property, Props, Component};
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
        Overflow,
        Position,
        LengthPercentageAuto,
        TextAlign,
    },
    geometry,
    Point,
    prelude::*,
};

pub fn style(comp: &mut Component, ui: &mut egui::Ui) -> Style {
    style_from_component(comp, style_from_ui(ui))
}

pub fn style_from_ui(ui: &mut egui::Ui) -> Style {
    let rect = ui.available_size();

    Style {
        size: Size {
            width: length(rect.x),
            height: length(rect.y),
        },
        ..Default::default()
    }
}

pub fn style_from_component(comp: &mut Component, base: Style) -> Style {
    if let Some(Property::Object(style)) = comp.props().get("style") {
        style_from_props(style, base)
    } else {
        base
    }
}

pub fn style_from_props(props: &Props, base: Style) -> Style {
    let mut style: Style = base;

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
    let size: Option<&Property> = props.get("size");

    if width.is_some() || height.is_some() {
        let width = width.map(|s| parse_dimension(&Into::<String>::into(s))).unwrap_or(Dimension::Auto);
        let height = height.map(|s| parse_dimension(&Into::<String>::into(s))).unwrap_or(Dimension::Auto);
        style.size = geometry::Size { width, height };
    } else if size.is_some() {
        let size = size.map(|s| parse_dimension(&Into::<String>::into(s))).unwrap_or(Dimension::Auto);
        style.size = geometry::Size { width: size, height: size };
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
