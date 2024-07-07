mod manager;

use crossbeam_channel::{unbounded, select, Receiver};

pub struct Tray {
    framebuffer: Vec<u8>,
    // icon_size: u32,
    // icon_quantity: u32,
    rx_tray: Receiver<manager::Event>,
}

// TODO SIGINT destroy_tray, click, fb

impl Tray {
    pub fn new(ctx: egui::Context) -> Self {
        let (tx_tray, rx_tray) = crossbeam_channel::unbounded();

        std::thread::spawn(move || {
            let (tx_event, rx_event) = crossbeam_channel::unbounded();

            let mut manager = manager::Manager::new(
                ctx,
                tx_tray,
            );

            let clonn = manager.conn.clone();
            std::thread::spawn(move || {
                loop {
                    if let Ok(event) = clonn.wait_for_event() {
                        tx_event.send(event).ok();
                    }
                }
            });

            loop {
                select! {
                    recv(rx_event) -> event => {
                        if let Ok(event) = event {
                            manager.handle_event(event);
                        }
                    }
                }
            }
        });

        Tray {
            framebuffer: vec![],
            // icon_size: 40,
            // icon_quantity: 0,
            rx_tray,
        }
    }

    pub fn click() {}

    pub fn signals(&self) {
        if let Ok(event) = self.rx_tray.try_recv() {
            event;
        }
    }
}
