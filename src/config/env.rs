use std::env;
use std::sync::OnceLock;

fn get_xdg(path: &str, default: &str) -> String {
    if let Ok(xdg_path) = env::var(path) {
        format!("{}/{}", xdg_path, crate::config::NAME)
    } else {
        let home = env::var("HOME").expect("POSIX requires you to set $HOME");
        format!("{}/{}/{}", home, default, crate::config::NAME)
    }
}

static NO_COLOR: OnceLock<bool> = OnceLock::new();
static BSPWM_SOCKET: OnceLock<String> = OnceLock::new();
static CAKEYBAR_SOCKET: OnceLock<String> = OnceLock::new();
static CONFIG_DIR: OnceLock<String> = OnceLock::new();
static CACHE_DIR: OnceLock<String> = OnceLock::new();

pub fn no_color() -> &'static bool {
    NO_COLOR.get_or_init(|| env::var("NO_COLOR").is_ok())
}

pub fn bspwm_socket() -> &'static String {
    BSPWM_SOCKET.get_or_init(|| env::var("BSPWM_SOCKET").unwrap_or_else(|_| "/tmp/bspwm_0_0-socket".to_string()))
}

pub fn cakeybar_socket() -> &'static String {
    CAKEYBAR_SOCKET.get_or_init(|| env::var("CAKEYBAR_SOCKET").unwrap_or_else(|_| "/tmp/cakeybar".to_string()))
}

pub fn config_dir() -> &'static String {
    CONFIG_DIR.get_or_init(|| get_xdg("XDG_CONFIG_HOME", ".config"))
}

pub fn cache_dir() -> &'static String {
    CACHE_DIR.get_or_init(|| get_xdg("XDG_CACHE_HOME", ".cache"))
}
