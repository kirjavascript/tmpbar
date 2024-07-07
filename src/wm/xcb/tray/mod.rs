mod manager;
mod signal_hook;

use manager::{Manager, TrayEvent, ProxyAction};

use crossbeam_channel::{unbounded, select, Receiver, Sender, tick};

pub struct Tray {
    pub framebuffer: Vec<u8>,
    pub icon_size: u32,
    pub icon_quantity: u32,
    rx_tray: Receiver<TrayEvent>,
    tx_proxy: Sender<ProxyAction>,
}

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

            let rx_signal = signal_hook::hook();

            let fb_tick = tick(std::time::Duration::from_millis(800));

            loop {
                // TODO: use Select to fix lint issue
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
                    },
                    recv(rx_signal) -> _ => {
                        manager.handle_action(ProxyAction::Destroy);
                    },
                    recv(fb_tick) -> _ => {
                        manager.handle_action(ProxyAction::PollFB);
                    },
                }
            }
        });

        Tray {
            framebuffer: vec![],
            icon_size: 40,
            icon_quantity: 0,
            rx_tray,
            tx_proxy,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.icon_size * self.icon_quantity, self.icon_size)
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
                TrayEvent::Framebuffer(fb, qty) => {
                    self.framebuffer = fb;
                    self.icon_quantity = qty;
                },
            }
        }
    }
}
