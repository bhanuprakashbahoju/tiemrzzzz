#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console on Windows in release

mod app;
mod display;
mod timer;

use app::TimerApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 400.0])
            .with_min_inner_size([180.0, 80.0])
            .with_title("Timer")
            .with_resizable(true)
            .with_always_on_top()
            .with_transparent(true), // Enable transparency
        ..Default::default()
    };

    eframe::run_native(
        "Timer",
        options,
        Box::new(|cc| Ok(Box::new(TimerApp::new(cc)))),
    )
}
