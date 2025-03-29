pub fn web_color_to_u32(color: &str) -> Option<u32> {
    let color = color.trim();

    if color.starts_with('#') {
        return parse_hex_color(&color[1..]);
    }

    if color.to_lowercase().starts_with("rgb(") && color.ends_with(')') {
        return parse_rgb_color(&color[4..color.len()-1]);
    }

    if color.to_lowercase().starts_with("rgba(") && color.ends_with(')') {
        return parse_rgba_color(&color[5..color.len()-1]);
    }

    None
}

fn parse_hex_color(hex: &str) -> Option<u32> {
    match hex.len() {
        // #RGB
        3 => {
            let r = u32::from_str_radix(&hex[0..1], 16).ok()?;
            let g = u32::from_str_radix(&hex[1..2], 16).ok()?;
            let b = u32::from_str_radix(&hex[2..3], 16).ok()?;

            Some((r << 28) | (r << 24) | (g << 20) | (g << 16) | (b << 12) | (b << 8) | 0xFF)
        },
        // #RRGGBB
        6 => {
            let color = u32::from_str_radix(hex, 16).ok()?;
            Some((color << 8) | 0xFF)
        },
        // #RRGGBBAA
        8 => {
            let rgb = u32::from_str_radix(&hex[0..6], 16).ok()?;
            let alpha = u32::from_str_radix(&hex[6..8], 16).ok()?;
            Some((rgb << 8) | alpha)
        },
        _ => None
    }
}

fn parse_rgb_color(rgb: &str) -> Option<u32> {
    let parts: Vec<&str> = rgb.split(',').collect();
    if parts.len() != 3 {
        return None;
    }

    let r = parts[0].trim().parse::<u32>().ok()?;
    let g = parts[1].trim().parse::<u32>().ok()?;
    let b = parts[2].trim().parse::<u32>().ok()?;

    if r > 255 || g > 255 || b > 255 {
        return None;
    }

    Some((r << 24) | (g << 16) | (b << 8) | 0xFF)
}

fn parse_rgba_color(rgba: &str) -> Option<u32> {
    let parts: Vec<&str> = rgba.split(',').collect();
    if parts.len() != 4 {
        return None;
    }

    let r = parts[0].trim().parse::<u32>().ok()?;
    let g = parts[1].trim().parse::<u32>().ok()?;
    let b = parts[2].trim().parse::<u32>().ok()?;

    let a_str = parts[3].trim();
    let a = if a_str.contains('.') {
        let a_float = a_str.parse::<f32>().ok()?;
        if a_float < 0.0 || a_float > 1.0 {
            return None;
        }
        (a_float * 255.0).round() as u32
    } else {
        let a_int = a_str.parse::<u32>().ok()?;
        if a_int > 255 {
            return None;
        }
        a_int
    };

    if r > 255 || g > 255 || b > 255 {
        return None;
    }

    Some((r << 24) | (g << 16) | (b << 8) | a)
}
