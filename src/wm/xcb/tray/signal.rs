
// TODO: signal hook

use std::io::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use signal_hook::consts::signal::*;
use signal_hook::flag as signal_flag;



    let term = Arc::new(AtomicUsize::new(0));
    const SIGTERM_U: usize = SIGTERM as usize;
    const SIGINT_U: usize = SIGINT as usize;
    const SIGQUIT_U: usize = SIGQUIT as usize;
    signal_flag::register_usize(SIGTERM, Arc::clone(&term), SIGTERM_U).unwrap();
    signal_flag::register_usize(SIGINT, Arc::clone(&term), SIGINT_U).unwrap();
    signal_flag::register_usize(SIGQUIT, Arc::clone(&term), SIGQUIT_U).unwrap();
        if term.load(Ordering::Relaxed) != 0 {
            println!("exit");
            return;
        }
