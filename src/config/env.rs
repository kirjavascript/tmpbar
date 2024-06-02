use std::env;
use std::sync::OnceLock;

static NO_COLOR: OnceLock<bool> = OnceLock::new();

pub fn no_color() -> &'static bool {
    NO_COLOR.get_or_init(|| env::var("NO_COLOR").is_ok())
}
