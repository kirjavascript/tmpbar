use eframe::egui;

#[macro_use]
mod util;

mod app;
mod components;
mod config;
mod global;
mod wm;

fn main() -> eframe::Result<()> {
    let args = config::args::get();

    if args.monitors {
        wm::monitor::print_info();
        std::process::exit(0);
    }

    if args.config.is_none() {
        config::args::usage();
        std::process::exit(1);
    }

    let path = args.config.as_ref().expect("unreachable").to_string();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("xcake-root")
            .with_inner_size([1.0, 1.0])
            .with_position(egui::Pos2 { x: 9999., y: 9999. })
            .with_decorations(false)
            .with_transparent(true)
            .with_window_level(egui::WindowLevel::AlwaysOnBottom)
            .with_window_type(egui::viewport::X11WindowType::Toolbar)
            ,..Default::default()
    };
    eframe::run_native(
        "🍰",
        native_options,
        Box::new(|cc| Box::new(app::TmpBar::new(
            cc,
            path,
        ))),
    )
}
