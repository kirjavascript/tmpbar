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

    if args.config.is_none() {
        config::args::usage();
        std::process::exit(1);
    }

    let path = args.config.as_ref().expect("unreachable");

    let (poll, config) = config::script::load(path);

    wm::xcb::window_patch();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("xcake-root")
            .with_inner_size([1.0, 1.0])
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
        Box::new(|cc| Box::new(app::TmpBar::new(
            cc,
            // poll,
            // config,
        ))),
    )
}
