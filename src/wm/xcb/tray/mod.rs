mod overlap;
mod manager;
mod signal_hook;
mod count_trays;

pub use count_trays::*;

use crate::util::web_color_to_u32;

use manager::{Manager, TrayEvent, ProxyAction};

use crossbeam_channel::{unbounded, select, Receiver, Sender};

pub struct Tray {
    pub dimensions: (u32, u32),
    old_pos: (i32, i32),
    old_size: u32,
    old_bgcolor: Option<String>,
    rx_tray: Receiver<TrayEvent>,
    tx_proxy: Sender<ProxyAction>,
}

impl Tray {
    pub fn new(ctx: egui::Context, has_tray: bool) -> Self {
        let (tx_tray, rx_tray) = unbounded();
        let (tx_proxy, rx_proxy) = unbounded();

        if has_tray {
            std::thread::spawn(move || {
                let (tx_event, rx_event) = unbounded();

                let mut manager = Manager::new(
                    ctx,
                    tx_tray,
                );

                let (tx_overlap, rx_overlap) = unbounded();

                std::thread::spawn(move || {
                    overlap::listen(manager.tray_window, tx_overlap);
                });

                let clonn = manager.conn.clone();
                std::thread::spawn(move || {
                    loop {
                        if let Ok(event) = clonn.wait_for_event() {
                            tx_event.send(event).ok();
                        }
                    }
                });

                let rx_signal = signal_hook::hook();

                loop {
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
                        recv(rx_overlap) -> is_overlapping => {
                            if let Ok(is_overlapping) = is_overlapping {
                                manager.handle_action(ProxyAction::Overlap(is_overlapping));
                            }
                        },
                    }
                }
            });
        }

        Tray {
            dimensions: (0, 0),
            old_pos: (0, 0),
            old_size: 20,
            old_bgcolor: None,
            rx_tray,
            tx_proxy,
        }
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        let (x1, y1) = self.old_pos;

        if x1 != x || y1 != y {
            self.tx_proxy.send(ProxyAction::Position(x, y)).ok();
            self.old_pos = (x, y);
        }
    }

    pub fn set_size(&mut self, size: u32) {
        if size != self.old_size {
            self.tx_proxy.send(ProxyAction::Size(size)).ok();
            self.old_size = size;
        }
    }

    pub fn set_bgcolor(&mut self, color: &str) {
        if Some(color) != self.old_bgcolor.as_deref() {
            self.old_bgcolor = Some(color.to_string());

            if let Some(number) = web_color_to_u32(color) {
                let number = number >> 8;
                self.tx_proxy.send(ProxyAction::BgColor(number)).ok();
            }
        }
    }

    pub fn signals(&mut self) {
        if let Ok(event) = self.rx_tray.try_recv() {
            match event {
                TrayEvent::Dimensions(x, y) => {
                    self.dimensions = (x, y);
                },
            }
        }
    }
}
