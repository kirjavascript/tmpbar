use crate::wm::monitor;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use mlua::{Value, Table};

static BAR_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(PartialEq)]
pub enum Position {
    Top,
    Bottom,
}

pub struct Bar {
    id: usize,
    pub position: Position,
    pub height: u32,
    pub monitor: monitor::Monitor,
    pub container: Component,
}

impl Bar {
    pub fn y(&self) -> i32 {
        if self.position == Position::Top {
            0
        } else {
            (self.monitor.height - self.height) as i32
        }
    }

    pub fn id(&self) -> String {
        "xcake-".to_string() + &self.id.to_string()
    }
}

pub type Props = HashMap<String, Property>;

#[derive(Debug, Clone)]
pub struct Component(String, Props);

impl Component {
    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn props(&mut self) -> &mut Props {
        &mut self.1
    }

    pub fn props_ref(&self) -> &Props {
        &self.1
    }
}

#[derive(Debug, Clone)]
pub enum Property {
    Component(Component),
    Object(HashMap<String, Property>),
    Array(Vec<Property>),
    Function(mlua::OwnedFunction),
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

pub fn parse_bars(lua: &mlua::Lua) -> mlua::Result<Vec<Bar>> {
    let monitors = monitor::list();
    let globals = lua.globals();

    let xcake_bars: Table = globals.get("xcake_bars")?;
    let mut bars = Vec::new();

    for pair in xcake_bars.pairs::<i32, Table>() {
        let (_, value) = pair?;

        let position: String = value.get("position").unwrap_or_else(|_| "top".to_string());
        let position = if position == "top".to_string() { Position::Top } else { Position::Bottom };
        let height: u32 = value.get("height").unwrap_or(25);

        let empty_table = lua.create_table()?;
        let monitor: Table = value.get("monitor").unwrap_or_else(|_| empty_table);

        let monitor = get_monitor(&monitor, &monitors);

        value.set("monitor", Value::Nil)?;
        value.set("height", Value::Nil)?;
        value.set("position", Value::Nil)?;

        let id = BAR_ID.fetch_add(1, Ordering::Relaxed);

        // default props for every component

        let default_props = create_default(
            format!("xcake-{}", id),
            monitor.index,
        );

        // get props for top level container

        let top_props = to_property(Value::Table(value), &default_props);

        let top_props = if let Property::Object(props) = top_props {
            props
        } else {
            HashMap::new()
        };

        bars.push(Bar {
            id,
            position,
            height,
            monitor: monitor.clone(),
            container: Component(
                "container".to_string(),
                top_props,
            ),
        });
    }

    Ok(bars)
}

pub fn copy_default(props: &Props) -> HashMap<String, Property> {
    let bar_id: String = props.get("_bar_id").unwrap_or_default().into();
    let monitor_index: i64 = props.get("_monitor_index").unwrap_or_default().into();
    create_default(bar_id, monitor_index as _)
}

fn create_default(bar_id: String, monitor_index: u32) -> HashMap<String, Property> {
    let mut default_props = HashMap::new();

    default_props.insert("_bar_id".to_string(), Property::String(bar_id));
    default_props.insert("_monitor_index".to_string(), Property::Integer(monitor_index as _));

    default_props
}

pub fn get_monitor<'a>(monitor: &Table, monitors: &'a Vec<monitor::Monitor>) -> &'a monitor::Monitor {
    let monitor_name: Result<String, mlua::prelude::LuaError> = monitor.get("name");
    let monitor_index = monitor.get("index").unwrap_or(1);
    let monitor = if let Ok(name) = monitor_name {
        monitors.iter().find(|monitor| { monitor.name == name })
    } else {
        None
    };

    monitor.unwrap_or_else(|| {
        monitors.get(monitor_index - 1).unwrap_or_else(|| {
            warn!("cannot find monitor, defaulting to {}", monitors[0].name);
            &monitors[0]
        })
    })
}

pub fn to_property(value: Value, default_props: &HashMap<String, Property>) -> Property {
    match value {
        Value::Nil => Property::Null,
        Value::Function(f) => Property::Function(f.into_owned()),
        Value::Boolean(b) => Property::Boolean(b),
        Value::Integer(i) => Property::Integer(i),
        Value::Number(n) => Property::Float(n),
        Value::String(s) => Property::String(s.to_str().unwrap().to_string()),
        Value::Table(t) => {
            let mut map = HashMap::new();
            let mut array = Vec::new();
            let mut is_array = true;
            for pair in t.pairs::<Value, Value>() {
                match pair {
                    Ok((key, val)) => {
                        let prop = to_property(val, default_props);
                        match key {
                            Value::Integer(idx) => {
                                if idx < 1 {
                                    is_array = false;
                                } else {
                                    array.push((idx as usize, prop));
                                }
                            },
                            Value::String(s) => {
                                map.insert(s.to_str().unwrap().to_string(), prop);
                                is_array = false;
                            }
                            _ => is_array = false,
                        }
                    }
                    Err(_) => is_array = false,
                }
            }
            let component_name: Option<String> = map.get("xcake_component").map(|c| c.into());

            if is_array {
                array.sort_by_key(|&(idx, _)| idx);
                Property::Array(array.into_iter().map(|(_, prop)| prop).collect())
            } else if component_name.is_some() {
                map.extend(default_props.iter().map(|(key, val)| {
                    (key.to_owned(), val.clone())
                }));
                Property::Component(Component(component_name.unwrap(), map))
            } else {
                Property::Object(map)
            }
        }
        _ => Property::Null,
    }
}

pub fn get_text(props: &Props, attr: &str) -> String {
    match props.get(attr) {
        Some(Property::Function(func)) => {
            func.call::<(), String>(())
                .unwrap_or_else(|e| e.to_string())
        }
        Some(Property::String(text)) => {
            text.to_owned()
        }
        _ => "".to_string()
    }
}

pub fn text_mut<'a>(props: &'a mut Props, attr: &str) -> &'a mut String {
    match props.get_mut(attr) {
        Some(Property::String(_)) => {},
        _ => { props.insert(attr.to_string(), Property::String("".to_string())); },
    };
    match props.get_mut(attr) {
        Some(Property::String(text)) => text,
        _ => unreachable!(),
    }
}

impl Default for &Property {
    fn default() -> Self {
        &Property::Null
    }
}

impl Into<String> for &Property {
    fn into(self) -> String {
        match self {
            Property::String(s) => s.to_owned(),
            Property::Integer(i) => i.to_string(),
            Property::Float(i) => i.to_string(),
            _ => String::default(),
        }
    }
}

impl Into<usize> for &Property {
    fn into(self) -> usize {
        match self {
            Property::Integer(i) => *i as _,
            Property::Float(f) => *f as _,
            _ => usize::default(),
        }
    }
}

impl Into<i64> for &Property {
    fn into(self) -> i64 {
        match self {
            Property::Integer(i) => *i,
            Property::Float(f) => *f as _,
            _ => i64::default(),
        }
    }
}

impl Into<i8> for &Property {
    fn into(self) -> i8 {
        match self {
            Property::Integer(i) => *i as _,
            Property::Float(f) => *f as _,
            _ => i8::default(),
        }
    }
}

impl Into<f64> for &Property {
    fn into(self) -> f64 {
        match self {
            Property::Float(f) => *f,
            Property::Integer(i) => *i as _,
            _ => f64::default(),
        }
    }
}

impl Into<f32> for &Property {
    fn into(self) -> f32 {
        match self {
            Property::Integer(i) => *i as _,
            Property::Float(f) => *f as _,
            _ => f32::default(),
        }
    }
}

impl Into<bool> for &Property {
    fn into(self) -> bool {
        match self {
            Property::Boolean(b) => *b,
            Property::Integer(i) => if *i > 0 { true } else { false },
            _ => bool::default(),
        }
    }
}
