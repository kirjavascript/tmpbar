pub fn format_bytes(bytes: f64) -> String {
    format_bytes_precision(bytes, None)
}

pub fn format_bytes_precision(bytes: f64, precision: Option<usize>) -> String {
    if bytes == 0. {
        return "0B".to_string()
    }
    const LEN: usize = 5;
    let sizes: [&str; LEN] = ["", "K", "M", "G", "T"];
    let index = ((bytes).ln() / 1024_f64.ln()).floor();
    let val = bytes / (1024_f64.powf(index));
    let index = index as usize;
    let suffix = if index < LEN { sizes[index] } else { "?" };

    let decimal_places = match precision {
        Some(p) => p,
        None => if index < 2 { 0 } else { 2 }
    };

    format!("{:.*}{}B", decimal_places, val, suffix)
}
