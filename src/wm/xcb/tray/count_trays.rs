use crate::config::{ConfigScript, Property, Component};

pub fn count_trays(script: &ConfigScript) -> usize {
    let mut count = 0;

    for bar in &script.bars {
        count += count_component_trays(script, &bar.container);
    }

    count
}

fn count_component_trays(script: &ConfigScript, component: &Component) -> usize {
    let mut count = 0;

    if component.name() == "tray" {
        count += 1;
    }

    let props = component.props_ref();

    for (_, prop) in props {
        match prop {
            Property::Component(child) => {
                count += count_component_trays(script, child);
            },
            Property::Object(obj) => {
                for (_, prop_value) in obj {
                    if let Property::Component(child) = prop_value {
                        count += count_component_trays(script, child);
                    }
                }
            },
            Property::Array(arr) => {
                for item in arr {
                    if let Property::Component(child) = item {
                        count += count_component_trays(script, child);
                    }
                }
            },
            _ => {}
        }
    }

    count
}
