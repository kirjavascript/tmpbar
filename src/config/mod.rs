pub mod args;
pub mod env;
pub mod script;
pub mod parse;
pub mod watch;

pub use parse::*;
pub use script::*;

pub static NAME: &str = env!("CARGO_PKG_NAME");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");
