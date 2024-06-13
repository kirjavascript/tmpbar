#[derive(Clone, Debug)]
pub struct Monitor {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}

pub fn print_info() {
    for (i, monitor) in list().iter().enumerate() {
        let &Monitor {ref name, width, height, x, y, .. } = monitor;

        println!("Monitor {i} {name} {width}x{height} x: {x} y: {y}");
    }
}

pub fn list_query() -> Result<Vec<Monitor>, xrandr::XrandrError> {
    let mut handle = xrandr::XHandle::open()?;

    Ok(handle.monitors()?.iter().map(|monitor| {
        let &xrandr::Monitor {ref name, width_px, height_px, x, y, .. } = monitor;

        Monitor {
            name: name.to_string(),
            x,
            y,
            width: width_px,
            height: height_px,
        }
    }).collect())
}

pub fn list() -> Vec<Monitor> {
    list_query().unwrap_or_else(|_| Vec::new())
}
