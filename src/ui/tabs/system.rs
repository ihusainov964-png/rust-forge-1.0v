// ui/tabs/system.rs — System tweaks tab (egui 0.22)

use egui::{RichText, Ui};
use crate::app::RustForgeApp;
use crate::ui::theme::*;
use crate::ui::widgets::*;

pub fn draw_system_tab(app: &mut RustForgeApp, ui: &mut Ui) {
    let tweaks = &mut app.config.system_tweaks;
    let hw     = &app.config.detected_hardware;

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(4.0);

        egui::Frame::none()
            .fill(C_WARNING.linear_multiply(0.10))
            .stroke(egui::Stroke::new(1.0, C_WARNING.linear_multiply(0.4)))
            .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                ui.label(RichText::new(
                    "⚠️  Системные твики изменяют настройки Windows через официальные API.\nВсе изменения ОБРАТИМЫ через кнопку «Откатить»."
                ).color(C_WARNING).size(12.0));
            });
        ui.add_space(10.0);

        section_header(ui, "Windows Performance", "🪟");
        egui::Grid::new("win_tweaks").num_columns(1).spacing([0.0, 8.0]).show(ui, |ui| {
            ui.horizontal(|ui| {
                checkbox_tip(ui, &mut tweaks.ultimate_power_plan, "Ultimate Performance Power Plan",
                    "Убирает ограничения CPU clock speed между кадрами.\nДаёт 5–15% CPU производительности.\npowercfg -duplicatescheme e9a42b02...");
                badge(ui, "РЕКОМЕНДУЕТСЯ", C_SUCCESS);
            });
            ui.end_row();
            ui.horizontal(|ui| {
                checkbox_tip(ui, &mut tweaks.disable_xbox_game_bar, "Отключить Xbox Game Bar",
                    "Game Bar работает в фоне и перехватывает Win+G.\nОтключение освобождает ресурсы.");
                badge(ui, "РЕКОМЕНДУЕТСЯ", C_SUCCESS);
            });
            ui.end_row();
            checkbox_tip(ui, &mut tweaks.game_mode, "Windows Game Mode",
                "AutoGameMode — Windows приоритизирует игровой процесс.");
            ui.end_row();
            ui.label(RichText::new("Hardware-Accelerated GPU Scheduling (HAGS)").size(13.0));
            ui.label(RichText::new("  → Параметры Windows → Экран → Графика → Стандартные параметры графики").color(C_TEXT_DIM).size(11.0));
            ui.label(RichText::new("  → Требует перезагрузки. Полезно на RTX 30xx+ и RX 6000+.").color(C_TEXT_DIM).size(11.0));
            ui.end_row();
        });

        ui.add_space(10.0);
        section_header(ui, "Ваше железо", "💻");
        egui::Frame::none()
            .fill(C_METAL)
            .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                let (tier, tier_color) = crate::core::hardware::get_hardware_tier(hw);
                info_row(ui, "CPU", &format!("{} ({} cores)", hw.cpu_name, hw.cpu_cores), None);
                info_row(ui, "GPU", &hw.gpu_name, None);
                info_row(ui, "RAM", &format!("{} MB  ({} GB)", hw.ram_total_mb, hw.ram_total_mb / 1024), None);
                info_row(ui, "OS",  &hw.os_version, None);
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Tier:").color(C_TEXT_DIM).size(13.0));
                    ui.label(RichText::new(tier).color(tier_color).size(13.0).strong());
                });
            });

        if hw.is_nvidia {
            ui.add_space(10.0);
            section_header(ui, "NVIDIA Оптимизации", "🟢");
            egui::Frame::none()
                .fill(C_METAL)
                .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
                .rounding(egui::Rounding::same(5.0))
                .show(ui, |ui| {
                    for tip in crate::core::tweaks::get_nvidia_tips() {
                        ui.label(RichText::new(&tip).size(12.0).color(C_TEXT));
                    }
                });
        }

        if hw.is_amd {
            ui.add_space(10.0);
            section_header(ui, "AMD Оптимизации", "🔴");
            egui::Frame::none()
                .fill(C_METAL)
                .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
                .rounding(egui::Rounding::same(5.0))
                .show(ui, |ui| {
                    for tip in &[
                        "📌 AMD Software → Anti-Lag → Enable",
                        "📌 Radeon Image Sharpening → On",
                        "📌 AMD FSR 2/3 → Quality или Balanced в настройках Rust",
                        "📌 Enhanced Sync вместо VSync → меньше тиринга и латентности",
                        "📌 EXPO/XMP в BIOS — важно для Ryzen систем!",
                    ] {
                        ui.label(RichText::new(*tip).size(12.0).color(C_TEXT));
                    }
                });
        }

        ui.add_space(10.0);
        section_header(ui, "Статус системы", "📊");
        egui::Frame::none()
            .fill(C_METAL)
            .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                for (key, val) in crate::core::tweaks::get_windows_perf_info() {
                    info_row(ui, &key, &val, None);
                }
            });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if action_button(ui, "⚡  Применить твики").clicked() {
                app.pending_action = Some(crate::app::PendingAction::ApplyTweaks);
            }
            ui.add_space(8.0);
            if secondary_button(ui, "🔄 Откатить всё").clicked() {
                app.pending_action = Some(crate::app::PendingAction::RevertTweaks);
            }
        });
        if tweaks.tweaks_applied {
            ui.add_space(4.0);
            ui.label(RichText::new("✅ Твики применены").color(C_SUCCESS).size(12.0));
        }
    });
}
