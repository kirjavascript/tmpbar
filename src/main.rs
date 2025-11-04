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

    if args.profiler {
        start_puffin_server();
    }

    let path = args.config.as_ref().expect("unreachable").to_string();

    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_title("xcake-root")
            .with_inner_size([1.0, 1.0])
            .with_position(egui::Pos2 { x: 99999., y: 99999. })
            .with_decorations(false)
            .with_transparent(true)
            .with_window_level(egui::WindowLevel::AlwaysOnBottom)
            .with_window_type(egui::viewport::X11WindowType::Toolbar)
            ,
        ..Default::default()
    };
    eframe::run_native(
        "🍰",
        native_options,
        Box::new(|cc| Ok(Box::new(app::TmpBar::new(
            cc,
            path,
        )))),
    )
}

fn start_puffin_server() {
    puffin::set_scopes_on(true);

    match puffin_http::Server::new("127.0.0.1:8585") {
        Ok(puffin_server) => {
            info!("run: cargo install puffin_viewer && puffin_viewer --url 127.0.0.1:8585");

            std::process::Command::new("puffin_viewer")
                .arg("--url")
                .arg("127.0.0.1:8585")
                .spawn()
                .ok();

            #[expect(clippy::mem_forget)]
            std::mem::forget(puffin_server);
        }
        Err(err) => {
            error!("Failed to start puffin server: {err}");
        }
    }
}
