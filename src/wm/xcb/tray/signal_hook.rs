
use crossbeam_channel::{Receiver, bounded};
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use std::thread;

pub fn hook() -> Receiver<i32> {
    let (sender, receiver) = bounded(1);

    thread::spawn(move || {
        let mut signals = Signals::new(&[SIGINT, SIGTERM, SIGQUIT]).expect("signals iterator broke :(");

        for signal in signals.forever() {
            match signal {
                SIGINT | SIGTERM | SIGQUIT => {
                    if sender.send(signal).is_err() {
                        break;
                    }
                },
                _ => unreachable!(),
            }
        }
    });

    receiver
}
