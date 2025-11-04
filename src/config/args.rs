use gumdrop::{Options, ParsingStyle};
use std::env;

#[derive(Debug, Options)]
pub struct Args {
    help: bool,
    #[options(help = "Specify a config path", meta = "[FILE]")]
    pub config: Option<String>,
    #[options(help = "Shows information about monitors")]
    pub monitors: bool,
    #[options(help = "Enable debug profiler")]
    pub profiler: bool,
}

pub fn get() -> Args {
    let args: Vec<String> = env::args().collect();

    let split = args.iter().position(|x| x == "--");

    let args = match split {
        Some(i) => &args[1..i],
        None => &args[1..],
    };

    match Args::parse_args(args, ParsingStyle::AllOptions) {
        Ok(args) => {
            if args.help_requested() {
                eprintln!("{} {}\n", super::NAME, super::VERSION);
                Args::parse_args_default_or_exit();
            }
            args
        },
        Err(err) => {
            error!("{}", err);
            std::process::exit(0);
        },
    }
}

pub fn dashdash_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    let split = args.iter().position(|x| x == "--");

    match split {
        Some(i) => args[i + 1..].to_vec(),
        None => vec![],
    }
}

pub fn usage() {
    eprintln!("{}", Args::usage());
}
