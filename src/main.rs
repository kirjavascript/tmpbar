use eframe::egui;

#[macro_use]
mod util;

mod app;
mod config;
mod wm;

fn main() -> eframe::Result<()> {
    let args = config::args::get();

    if args.monitors {
        wm::monitor::print_info().expect("XrandrError");
        std::process::exit(0);
    }

    info!("{:#?}", args);

    wm::xcb::window_patch();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("xcake-root")
            .with_inner_size([10.0, 10.0])
            .with_window_level(egui::WindowLevel::AlwaysOnBottom)
            .with_decorations(false)
            .with_resizable(false)
            .with_transparent(true)
            .with_window_type(egui::viewport::X11WindowType::Toolbar)
            ,
            ..Default::default()
    };
    eframe::run_native(
        "🍰",
        native_options,
        Box::new(|cc| Box::new(app::TmpBar::new(cc))),
    )
}
