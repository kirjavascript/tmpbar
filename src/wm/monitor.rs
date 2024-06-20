use display_info::DisplayInfo;

#[derive(Clone, Debug)]
pub struct Monitor {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
}

pub fn print_info() {
    for (i, monitor) in list().iter().enumerate() {
        let &Monitor {ref name, width, height, x, y, .. } = monitor;

        println!("Monitor {i} {name} {width}x{height} x: {x} y: {y}");
    }
}

pub fn list() -> Vec<Monitor> {
    let list = DisplayInfo::all().unwrap_or_else(|_| {
        warn!("didn't find any monitors");
        Vec::new()
    });

    list.iter().map(|m| {
        Monitor {
            name: m.name.to_string(),
            x: m.x,
            y: m.y,
            width: m.width,
            height: m.height
        }
    }).collect()
}
