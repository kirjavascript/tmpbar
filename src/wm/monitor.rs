pub fn print_info() -> Result<(), xrandr::XrandrError> {
    let mut handle = xrandr::XHandle::open()?;

    for (i, monitor) in handle.monitors()?.iter().enumerate() {
        let &xrandr::Monitor {ref name, width_px, height_px, x, y, .. } = monitor;

        println!("Monitor {i} {name} {width_px}x{height_px} x: {x} y: {y}");
    }

    Ok(())
}
