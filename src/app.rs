use crate::config::ConfigScript;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TmpBar {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TmpBar {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

use eframe::egui;

impl TmpBar {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, config: ConfigScript) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TmpBar {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // crate::wm::xcb::window_patch();
        // ctx.request_repaint_after(std::time::Duration::from_secs(1));

        // println!("render");
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:

        //     egui::menu::bar(ui, |ui| {
        //         // NOTE: no File->Quit on web pages!
        //         let is_web = cfg!(target_arch = "wasm32");
        //         if !is_web {
        //             ui.menu_button("File", |ui| {
        //                 if ui.button("Quit").clicked() {
        //                     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        //                 }
        //             });
        //             ui.add_space(16.0);
        //         }

        //         egui::widgets::global_dark_light_mode_buttons(ui);
        //     });
        // });


        // TODO: change to deferred
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("immediate_viewport0"),
            egui::ViewportBuilder::default()
            .with_title("xcake-0")
            .with_position(egui::Pos2 { x: 0., y: 0. })
            .with_window_type(egui::viewport::X11WindowType::Dock)
            .with_inner_size([1920.0, 20.0])
            .with_transparent(true),
            |ctx, class| {
                // assert!(
                //     class == egui::ViewportClass::Immediate,
                //     "This egui backend doesn't support multiple viewports"
                // );

                // TODO: change to Window
                egui::CentralPanel::default()
                    .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT))
                    .show(ctx, |ui| {
                    ui.label("bar 0");
                });

            },
        );

        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("immediate_viewport1"),
            egui::ViewportBuilder::default()
            .with_title("xcake-1")
            .with_window_type(egui::viewport::X11WindowType::Dock)
            .with_position(egui::Pos2 { x: 0., y: 1060. })
            .with_transparent(true)
            .with_inner_size([1920.0, 20.0]),
            |ctx, class| {
                // assert!(
                //     class == egui::ViewportClass::Immediate,
                //     "This egui backend doesn't support multiple viewports"
                // );

                egui::CentralPanel::default()
                    .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT))
                    .show(ctx, |ui| {
                    ui.label("bar 1");
                });

            },
        );

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     // The central panel the region left after adding TopPanel's and SidePanel's
        //     ui.heading("eframe template");

        //     ui.horizontal(|ui| {
        //         ui.label("Write something: ");
        //         ui.text_edit_singleline(&mut self.label);
        //     });

        //     ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
        //     if ui.button("Increment").clicked() {
        //         self.value += 1.0;
        //     }

        //     ui.separator();

        //     ui.add(egui::github_link_file!(
        //         "https://github.com/emilk/eframe_template/blob/main/",
        //         "Source code."
        //     ));

        //     ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //         powered_by_egui_and_eframe(ui);
        //         egui::warn_if_debug_build(ui);
        //     });
        // });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Color32::TRANSPARENT.to_normalized_gamma_f32()
    }
}
