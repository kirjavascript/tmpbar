use gumdrop::{Options, ParsingStyle};
use std::env;

#[derive(Debug, Options)]
pub struct Args {
    help: bool,
    #[options(help = "Specify a config path", meta = "[FILE]")]
    pub config: Option<String>,
    #[options(help = "Shows information about monitors")]
    pub monitors: bool,
}

pub fn get() -> Args {
    let args: Vec<String> = env::args().collect();

    match Args::parse_args(&args[1..], ParsingStyle::AllOptions) {
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

pub fn usage() {
    eprintln!("{}", Args::usage());
}
