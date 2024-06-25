use inotify::{Inotify, WatchMask};
use crate::util::Signal;

pub fn init(path: &str, reload_signal: Signal<()>) {
    let path = path.to_string();

    std::thread::spawn(move || {
        let mut inotify = Inotify::init().expect("error loading inotify");
        let path = std::path::Path::new(&path);

        inotify
            .watches()
            .add(path, WatchMask::MODIFY)
            .expect("error adding watcher");

        let mut buffer = [0; 1024];
        loop {
            match inotify.read_events(&mut buffer) {
                Ok(_) => {
                    reload_signal.send(());
                }
                Err(_) => {},
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });
}
