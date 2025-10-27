use crate::{global, config};
use crate::wm::xcb::Tray;
use eframe::egui;

pub struct TmpBar {
    config: config::ConfigScript,
    global: global::Global,
}

impl TmpBar {
    pub fn new(cc: &eframe::CreationContext<'_>, path: String) -> Self {
        let path = match std::fs::canonicalize(std::path::Path::new(&path)) {
            Ok(path) => path,
            Err(error) => {
                error!("{}", error);
                std::process::exit(0);
            },
        };

        let gl = cc
            .gl
            .as_ref()
            .expect("glow backend expected");

        let mut global = global::Global::new(
            &path,
            cc.egui_ctx.clone(),
            gl.clone(),
        );

        // [!] cwd is now set to the location of the config

        let config = config::script::init(
            &path,
            cc.egui_ctx.clone(),
            &global.lua
        );

        global.set_theme(&config);

        crate::wm::xcb::window_patch(&config);

        let trays = crate::wm::xcb::count_trays(&config);

        if trays > 1 {
            warn!("multiple trays in config ({})", trays);
        } else if trays == 1 {
            global.tray = Some(Tray::new(cc.egui_ctx.clone()));
        }

        egui_extras::install_image_loaders(&cc.egui_ctx);

        TmpBar {
            config,
            global,
        }
    }
}

impl eframe::App for TmpBar {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.config.reload_signal.consume() {
            info!("reloading config");

            if let Err(err) = self.config.reload(&self.global.lua) {
                error!("{}", err);
            } else {
                self.global.set_theme(&self.config);
                crate::wm::xcb::window_patch(&self.config);
                ctx.memory_mut(|mem| mem.data.clear());
            }
        }

        self.global.frame();

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

                    let min_interval: usize = {
                        match bar.container.props().get("min_interval") {
                            Some(config::Property::Integer(num)) => *num as usize,
                            Some(config::Property::Float(num)) => *num as usize,
                            Some(config::Property::Boolean(_)) => 0,
                            _ => 1000,
                        }
                    };

                    ctx.request_repaint_after(std::time::Duration::from_millis(min_interval as _));

                    egui::CentralPanel::default()
                        .frame(egui::Frame::new().fill(egui::Color32::TRANSPARENT))
                        .show(ctx, |ui| {
                            // taffy: enable multipass
                            ctx.options_mut(|options| {
                                options.max_passes = std::num::NonZeroUsize::new(3).unwrap();
                            });

                            // taffy: disable text wrapping
                            ctx.style_mut(|style| {
                                style.wrap_mode = Some(egui::TextWrapMode::Extend);
                            });

                            crate::components::render(
                                &mut bar.container,
                                ui,
                                &mut self.global
                            );
                    });
                },
            );
        }
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Color32::TRANSPARENT.to_normalized_gamma_f32()
    }
}
