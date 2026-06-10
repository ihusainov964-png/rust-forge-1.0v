// ui/tabs/about.rs — About tab (egui 0.22)

use egui::{RichText, Ui};
use crate::app::RustForgeApp;
use crate::ui::theme::*;
use crate::ui::widgets::*;

pub fn draw_about_tab(_app: &mut RustForgeApp, ui: &mut Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(8.0);
        ui.vertical_centered(|ui| {
            ui.label(RichText::new("⚙ RUSTFORGE").color(C_RUST).size(32.0).strong());
            ui.label(RichText::new("Ultimate Rust Game Optimizer").color(C_TEXT_DIM).size(14.0));
            ui.label(RichText::new("v1.0.0 — 2026").color(C_TEXT_DIM).size(12.0));
        });
        ui.add_space(16.0);

        section_header(ui, "Что делает RustForge?", "📋");
        egui::Frame::none()
            .fill(C_METAL)
            .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                for line in &[
                    "✅ Управляет Steam Launch Options для Rust",
                    "✅ Записывает client.cfg с оптимальными настройками",
                    "✅ Применяет легальные Windows-твики (Power Plan, Game Bar и т.д.)",
                    "✅ Профили PVP / Max FPS / High Visibility / Custom",
                    "✅ Автоопределяет железо и рекомендует настройки",
                    "✅ Создаёт бэкап перед любыми изменениями",
                ] {
                    ui.label(RichText::new(*line).size(13.0).color(C_TEXT));
                }
            });

        ui.add_space(10.0);
        section_header(ui, "Ожидаемый прирост FPS", "📈");
        egui::Frame::none()
            .fill(C_METAL)
            .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                ui.label(RichText::new("Реалистичные ожидания (зависит от железа):").color(C_TEXT_DIM).size(12.0));
                ui.add_space(4.0);
                egui::Grid::new("fps_table").num_columns(2).spacing([20.0, 4.0]).show(ui, |ui| {
                    for (setting, gain) in &[
                        ("Graphics Quality 6→2",     "+30–50 FPS"),
                        ("Shadows OFF",               "+15–30 FPS"),
                        ("Water Quality 3→0",         "+10–20 FPS"),
                        ("LOD Bias снижение",          "+20–50% у баз"),
                        ("Ultimate Power Plan",        "+5–15%"),
                        ("Launch Options комплекс",    "+2–10 FPS"),
                        ("Water Reflections OFF",      "+10–15 FPS"),
                        ("Grass Quality 3→0",          "+5–15 FPS"),
                    ] {
                        ui.label(RichText::new(*setting).color(C_TEXT_DIM).size(12.0));
                        ui.label(RichText::new(*gain).color(C_SUCCESS).size(12.0).strong());
                        ui.end_row();
                    }
                });
                ui.add_space(4.0);
                ui.label(RichText::new(
                    "⚠️ Rust — CPU-bound игра. Апгрейд CPU даёт больше FPS, чем GPU.\nРезультаты варьируются в зависимости от конфигурации системы."
                ).color(C_WARNING).size(11.0).italics());
            });

        ui.add_space(10.0);
        section_header(ui, "Безопасность и легальность", "🛡️");
        egui::Frame::none()
            .fill(C_METAL)
            .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                for line in &[
                    "✅ Нет чтения/записи памяти игровых процессов",
                    "✅ Нет инъекций DLL или кода",
                    "✅ Нет кейлоггинга или сниффинга трафика",
                    "✅ Только launch options + config файлы + официальные Windows API",
                    "✅ Не нарушает EAC (Easy Anti-Cheat)",
                    "✅ Открытый исходный код — проверь сам на GitHub",
                ] {
                    ui.label(RichText::new(*line).size(13.0).color(C_SUCCESS));
                }
            });

        ui.add_space(10.0);
        section_header(ui, "Полезные ссылки", "🔗");
        ui.horizontal(|ui| {
            if secondary_button(ui, "📖 GitHub репозиторий").clicked() {
                open_url("https://github.com/your-repo/rust-forge");
            }
            ui.add_space(8.0);
            if secondary_button(ui, "🎮 Rust FPS Guide").clicked() {
                open_url("https://www.rustafied.com");
            }
        });
    });
}

fn open_url(url: &str) {
    #[cfg(target_os = "windows")]
    { let _ = std::process::Command::new("cmd").args(["/c", "start", "", url]).spawn(); }
    #[cfg(target_os = "linux")]
    { let _ = std::process::Command::new("xdg-open").arg(url).spawn(); }
}
