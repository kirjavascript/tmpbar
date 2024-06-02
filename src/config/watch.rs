use inotify::{Inotify, WatchMask};
use std::sync::mpsc::{channel, Receiver};

pub fn init(path: &str) -> Receiver<()> {
    let (tx, rx) = channel();

    let path = path.to_string();

    std::thread::spawn(move || {
        let mut inotify = Inotify::init().expect("error loading inotify");
        let path = std::path::Path::new(&path);

        inotify
            .watches()
            .add(path, WatchMask::MODIFY | WatchMask::CLOSE)
            .expect("error adding watcher");

        let mut buffer = [0; 1024];
        loop {
            match inotify.read_events(&mut buffer) {
                Ok(_) => {
                    tx.send(()).inspect_err(|e| error!("{e}")).unwrap();
                }
                Err(_) => {},
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    rx
}
