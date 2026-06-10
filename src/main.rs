// RustForge — Ultimate Rust Game Optimizer & Launcher
// 100% legal: only launch options, config files, system tweaks via official Windows APIs
// No memory reading, no cheats, no injections. Safe with EAC.

#![windows_subsystem = "windows"]

mod app;
mod config;
mod core;
mod ui;

use app::RustForgeApp;
use eframe::NativeOptions;

fn main() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();

    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(1100.0, 720.0)),
        min_window_size:     Some(egui::vec2(900.0,  600.0)),
        icon_data:           Some(make_icon()),
        resizable:           true,
        decorated:           true,
        ..Default::default()
    };

    eframe::run_native(
        "RustForge — Ultimate Optimizer",
        options,
        Box::new(|cc| Box::new(RustForgeApp::new(cc))),
    );
}

fn make_icon() -> eframe::IconData {
    let size = 32usize;
    let mut rgba = vec![0u8; size * size * 4];
    for y in 0..size {
        for x in 0..size {
            let idx = (y * size + x) * 4;
            let fx  = x as f32 / size as f32;
            let fy  = y as f32 / size as f32;
            let d   = ((fx - 0.5).powi(2) + (fy - 0.5).powi(2)).sqrt();
            if d < 0.48 {
                rgba[idx]     = (200.0 - fx * 60.0) as u8;
                rgba[idx + 1] = (80.0  - fy * 40.0) as u8;
                rgba[idx + 2] = 20;
                rgba[idx + 3] = 255;
            }
        }
    }
    eframe::IconData { rgba, width: size as u32, height: size as u32 }
}

use egui;
