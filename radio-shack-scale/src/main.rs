#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    use egui::Vec2;
    tracing_subscriber::fmt::init();

    let size = Some(Vec2 { x: 320., y: 220. });
    let native_options = eframe::NativeOptions {
        min_window_size: size,
        max_window_size: size,
        initial_window_size: size,
        ..Default::default()
    };
    eframe::run_native(
        "Radio Shack Scale",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    );
}
