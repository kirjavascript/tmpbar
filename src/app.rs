use crate::{global, config};
use eframe::egui;

pub struct TmpBar {
    config: config::ConfigScript,
    global: global::Global,
}

impl TmpBar {
    pub fn new(cc: &eframe::CreationContext<'_>, path: String) -> Self {
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        let config = config::script::init(&path, cc.egui_ctx.clone());
        let global = global::Global::new(cc.egui_ctx.clone());

        egui_extras::install_image_loaders(&cc.egui_ctx);

        crate::wm::xcb::window_patch(&config);

        TmpBar {
            config,
            global,
        }
    }
}

impl eframe::App for TmpBar {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TODO:
        // colours
        // absolute positioning in stripbuilder
        // styles?
        // redo structure
        //
        // workspace data
        // input: pressing enter doesnt unfocus properly

        if self.config.reload_signal.has() {
            info!("reloading config");

            if let Err(err) = self.config.reload() {
                error!("{}", err);
            } else {
                crate::wm::xcb::window_patch(&self.config);
            }
        }

        self.global.signals(&self.config.lua);

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
