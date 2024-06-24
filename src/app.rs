use crate::config;
use eframe::egui;
use std::sync::Arc;

pub struct TmpBar {
    config: config::ConfigScript,
    poll_watch: Arc<dyn Fn() -> bool + Send>,
}

impl TmpBar {
    pub fn new(cc: &eframe::CreationContext<'_>, path: String) -> Self {
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let now = std::time::SystemTime::now();
        let (poll_watch, config) = config::script::load(&path);

        egui_extras::install_image_loaders(&cc.egui_ctx);

        info!("config loaded ({:?})", now.elapsed().unwrap());

        crate::wm::xcb::window_patch(&config);

        TmpBar {
            config,
            poll_watch: Arc::new(poll_watch),
        }
    }
}

impl eframe::App for TmpBar {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // rerender every half second minimum to poll config watcher
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
        // TODO: use repaint_signal
//https://github.com/lucasmerlin/hello_egui/tree/bd18788be1c0e7bad7bcc75f3088715fad1e0279/crates/egui_inbox
// TODO: fork egui::run_simple_native

        // TODO:
        // styles
        // push rendering (non-polling)
        // absolute positioning in stripbuilder
        // for input: https://docs.rs/xcb/latest/xcb/x/struct.GrabKeyboard.html & pass bar id to every component

        if (self.poll_watch)() {
            if let Err(err) = self.config.reload() {
                error!("{}", err);
            } else {
                crate::wm::xcb::window_patch(&self.config);
            }
        }

        for bar in self.config.bars.iter_mut() {
            let id = bar.id();

            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of(&id),
                egui::ViewportBuilder::default()
                .with_title(&id)
                .with_position(egui::Pos2 { x: bar.monitor.x as _, y: bar.y() as _ })
                .with_inner_size([bar.monitor.width as _, bar.height as _])
                .with_window_type(egui::viewport::X11WindowType::Dock)

                .with_window_level(egui::WindowLevel::AlwaysOnBottom)
                .with_decorations(false)
                .with_transparent(true),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default()
                        .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT))
                        .show(ctx, |ui| {
                            crate::components::render(&mut bar.container, ui);
                    });
                },
            );
        }
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Color32::TRANSPARENT.to_normalized_gamma_f32()
    }
}
