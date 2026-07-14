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

    // Peek at saved preference so the window icon matches the chosen theme
    // even before the egui context exists.
    let use_frieren_theme = config::load_and_peek_theme();

    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(1100.0, 720.0)),
        min_window_size:     Some(egui::vec2(900.0,  600.0)),
        icon_data:           Some(if use_frieren_theme { make_wanderer_icon() } else { make_icon() }),
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

/// Original "wanderer under the stars" icon — a soft crescent moon on a
/// deep night-lavender disc with three small sparkles. Entirely procedural
/// pixel art drawn by code below; not a trace, copy, or likeness of any
/// existing character, artwork, or franchise.
fn make_wanderer_icon() -> eframe::IconData {
    let size = 32usize;
    let mut rgba = vec![0u8; size * size * 4];

    let bg      = (30u8,  28u8,  40u8);   // deep night
    let bg_edge = (46u8,  42u8,  60u8);   // lighter night ring
    let moon    = (196u8, 184u8, 222u8);  // soft lavender moon
    let star    = (232u8, 220u8, 180u8);  // pale gold sparkle

    let cx = size as f32 / 2.0;
    let cy = size as f32 / 2.0;
    let r  = size as f32 * 0.48;

    // sparkle positions (x, y, half-size)
    let sparkles: [(f32, f32, f32); 3] = [
        (size as f32 * 0.72, size as f32 * 0.24, 1.6),
        (size as f32 * 0.80, size as f32 * 0.42, 1.0),
        (size as f32 * 0.66, size as f32 * 0.62, 1.2),
    ];

    for y in 0..size {
        for x in 0..size {
            let idx = (y * size + x) * 4;
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let d  = (dx * dx + dy * dy).sqrt();

            if d >= r {
                rgba[idx + 3] = 0; // transparent outside the disc
                continue;
            }

            // Background disc, brighter near the rim for a soft glow
            let edge_t = ((d / r) - 0.7).max(0.0) / 0.3;
            let (mut cr, mut cg, mut cb) = (
                bg.0 as f32 + (bg_edge.0 as f32 - bg.0 as f32) * edge_t,
                bg.1 as f32 + (bg_edge.1 as f32 - bg.1 as f32) * edge_t,
                bg.2 as f32 + (bg_edge.2 as f32 - bg.2 as f32) * edge_t,
            );

            // Crescent moon: a moon-disc minus an offset "bite" disc
            let mx = size as f32 * 0.40;
            let my = size as f32 * 0.50;
            let mr = size as f32 * 0.22;
            let moon_d = ((x as f32 - mx).powi(2) + (y as f32 - my).powi(2)).sqrt();

            let bite_x = mx + size as f32 * 0.10;
            let bite_y = my - size as f32 * 0.04;
            let bite_d = ((x as f32 - bite_x).powi(2) + (y as f32 - bite_y).powi(2)).sqrt();

            if moon_d < mr && bite_d > mr * 0.92 {
                cr = moon.0 as f32;
                cg = moon.1 as f32;
                cb = moon.2 as f32;
            }

            // Sparkles: small four-point stars (diamond blend)
            for (sx, sy, half) in sparkles.iter() {
                let sdx = (x as f32 - sx).abs();
                let sdy = (y as f32 - sy).abs();
                if sdx + sdy < *half {
                    let t = 1.0 - (sdx + sdy) / half.max(0.001);
                    cr = cr + (star.0 as f32 - cr) * t;
                    cg = cg + (star.1 as f32 - cg) * t;
                    cb = cb + (star.2 as f32 - cb) * t;
                }
            }

            rgba[idx]     = cr.clamp(0.0, 255.0) as u8;
            rgba[idx + 1] = cg.clamp(0.0, 255.0) as u8;
            rgba[idx + 2] = cb.clamp(0.0, 255.0) as u8;
            rgba[idx + 3] = 255;
        }
    }

    eframe::IconData { rgba, width: size as u32, height: size as u32 }
}

use egui;
