mod manager;

use manager::{Manager, TrayEvent, ProxyAction};

use crossbeam_channel::{unbounded, select, Receiver, Sender};

pub struct Tray {
    pub framebuffer: Vec<u8>,
    icon_size: u32,
    // icon_quantity: u32,
    rx_tray: Receiver<TrayEvent>,
    tx_proxy: Sender<ProxyAction>,
}

// TODO SIGINT destroy_tray

impl Tray {
    pub fn new(ctx: egui::Context) -> Self {
        let (tx_tray, rx_tray) = unbounded();
        let (tx_proxy, rx_proxy) = unbounded();

        std::thread::spawn(move || {
            let (tx_event, rx_event) = unbounded();

            let mut manager = Manager::new(
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
                // TODO: use docs to fix lint issue
                select! {
                    recv(rx_event) -> event => {
                        if let Ok(event) = event {
                            manager.handle_event(event);
                        }
                    },
                    recv(rx_proxy) -> action => {
                        if let Ok(action) = action {
                            manager.handle_action(action);
                        }
                    }
                }
            }
        });

        Tray {
            framebuffer: vec![],
            icon_size: 40,
            // icon_quantity: 0,
            rx_tray,
            tx_proxy,
        }
    }

    pub fn click(&self, button: u8, icon_index: usize) {
        self.tx_proxy.send(manager::ProxyAction::Click(
            button,
            icon_index
        )).ok();
    }

    pub fn signals(&mut self) {
        if let Ok(event) = self.rx_tray.try_recv() {
            match event {
                TrayEvent::Framebuffer(fb) => {
                    self.framebuffer = fb;

                    for c in self.framebuffer.chunks(self.icon_size as _) {
                        for c in c.chunks(4) {
                            print!("{:0>2X}{:0>2X}{:0>2X}",c[0],c[1],c[2]);
                        }
                        println!("");
                    }
                },
            }
        }
    }
}
