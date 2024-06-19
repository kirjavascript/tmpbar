use crate::wm::monitor;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use mlua::{Value, Table};

static BAR_ID: AtomicUsize = AtomicUsize::new(0);

pub struct ConfigScript {
    pub path: String,
    pub bars: Vec<Bar>,
}

#[derive(PartialEq)]
pub enum Position {
    Top,
    Bottom,
}

pub struct Bar {
    id: usize,
    pub position: Position,
    pub height: i32,
    pub monitor: monitor::Monitor,
    pub layout: Vec<Component>,
}

impl Bar {
    pub fn y(&self) -> i32 {
        if self.position == Position::Top {
            0
        } else {
            self.monitor.height - self.height
        }
    }

    pub fn id(&self) -> String {
        "xcake-".to_string() + &self.id.to_string()
    }
}

pub type Props = HashMap<String, Property>;

#[derive(Debug)]
pub struct Component(String, Props);

impl Component {
    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn props(&mut self) -> &mut Props {
        &mut self.1
    }
}

#[derive(Debug)]
pub enum Property {
    Component(Component),
    Function(mlua::OwnedFunction),
    String(String),
    Integer(i64),
    Float(f64),
    Array(Vec<Property>),
    Boolean(bool),
    Object(HashMap<String, Property>),
    Null,
}

pub fn parse_script(path: &str, lua: &mlua::Lua) -> mlua::Result<ConfigScript> {
    let monitors = monitor::list();
    let globals = lua.globals();

    let xcake_bars: Table = globals.get("xcake_bars")?;
    let mut bars = Vec::new();

    for pair in xcake_bars.pairs::<i32, Table>() {
        let (_, value) = pair?;

        let position: String = value.get("position").unwrap_or_else(|_| "top".to_string());
        let position = if position == "top".to_string() { Position::Top } else { Position::Bottom };
        let height: i32 = value.get("height").unwrap_or(25);

        let empty_table = lua.create_table()?;
        let monitor: Table = value.get("monitor").unwrap_or_else(|_| empty_table);
        let monitor_name: Result<String, mlua::prelude::LuaError> = monitor.get("name");
        let monitor_index = monitor.get("index").unwrap_or(1);
        let monitor = if let Ok(name) = monitor_name {
            monitors.iter().find(|monitor| { monitor.name == name })
        } else {
            None
        };
        let monitor = monitor.unwrap_or_else(|| {
            monitors.get(monitor_index - 1).unwrap_or_else(|| {
                warn!("cannot find monitor, defaulting to {}", monitors[0].name);
                &monitors[0]
            })
        });

        let empty_table = lua.create_table()?;
        let layout: mlua::Table = value.get("layout").unwrap_or_else(|_| empty_table);

        let mut components = Vec::new();

        for table in layout.sequence_values::<mlua::Table>() {
            let value = Value::Table(table?);
            if let Property::Component(component) = to_property(value) {
                components.push(component);
            }
        }

        bars.push(Bar {
            id: BAR_ID.fetch_add(1, Ordering::Relaxed),
            position,
            height,
            monitor: monitor.clone(),
            layout: components,
        });
    }

    Ok(ConfigScript {
        path: path.to_string(),
        bars,
    })
}

fn to_property(value: Value) -> Property {
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
            let mut component_name = None;
            for pair in t.pairs::<Value, Value>() {
                match pair {
                    Ok((key, val)) => {
                        let prop = to_property(val);
                        match key {
                            Value::Integer(idx) => {
                                if idx < 1 {
                                    is_array = false;
                                } else {
                                    if idx == 1 {
                                        if let Property::String(ref name) = prop {
                                            component_name = Some(name.clone())
                                        }
                                    }
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
            if is_array {
                array.sort_by_key(|&(idx, _)| idx);
                Property::Array(array.into_iter().map(|(_, prop)| prop).collect())
            } else if component_name.is_some() {
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
                .unwrap_or("[error function]".to_string())
        }
        Some(Property::String(text)) => {
            text.to_owned()
        }
        _ => "[error text]".to_string()
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
