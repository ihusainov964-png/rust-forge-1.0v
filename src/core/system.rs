// ui/tabs/system.rs — System tweaks + parasite killer (egui 0.22)

use egui::{RichText, Ui};
use crate::app::RustForgeApp;
use crate::ui::theme::*;
use crate::ui::widgets::*;

pub fn draw_system_tab(app: &mut RustForgeApp, ui: &mut Ui) {
    let hw           = app.config.detected_hardware.clone();
    let tweaks_applied = app.config.system_tweaks.tweaks_applied;

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(4.0);

        // ── Warning ───────────────────────────────────────────────────────────
        egui::Frame::none()
            .fill(C_WARNING.linear_multiply(0.10))
            .stroke(egui::Stroke::new(1.0, C_WARNING.linear_multiply(0.4)))
            .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                ui.label(RichText::new(
                    "⚠️  Все изменения через официальные Windows API. Полностью ОБРАТИМЫ кнопкой «Откатить всё»."
                ).color(C_WARNING).size(12.0));
            });
        ui.add_space(10.0);

        // ── Performance tweaks ────────────────────────────────────────────────
        section_header(ui, "Производительность Windows", "⚡");
        egui::Grid::new("perf_tweaks").num_columns(2).spacing([20.0, 7.0]).show(ui, |ui| {
            ui.horizontal(|ui| {
                checkbox_tip(ui, &mut app.config.system_tweaks.ultimate_power_plan,
                    "Ultimate Performance Power Plan",
                    "Убирает ограничения CPU clock.\n+5–15% CPU производительности в играх.");
                badge(ui, "ТОП", C_RUST);
            });
            ui.horizontal(|ui| {
                checkbox_tip(ui, &mut app.config.system_tweaks.game_mode,
                    "Windows Game Mode",
                    "Windows приоритизирует игровой процесс над фоновыми.");
                badge(ui, "ТОП", C_RUST);
            });
            ui.end_row();

            checkbox_tip(ui, &mut app.config.system_tweaks.disable_xbox_game_bar,
                "Отключить Xbox Game Bar",
                "Game Bar перехватывает Win+G и работает в фоне.\nОтключение экономит RAM и CPU.");
            checkbox_tip(ui, &mut app.config.system_tweaks.set_high_timer_res,
                "Таймер 1ms (timeBeginPeriod)",
                "Устанавливает разрешение системного таймера 1ms.\nДаёт плавный фреймтайм, меньше статтеров.");
            ui.end_row();

            checkbox_tip(ui, &mut app.config.system_tweaks.disable_fullscreen_optimizations,
                "Fullscreen Optimizations — инфо",
                "Отключается вручную: ПКМ на RustClient.exe → Свойства → Совместимость.");
            checkbox_tip(ui, &mut app.config.system_tweaks.hardware_gpu_scheduling,
                "HAGS — инфо",
                "Параметры Windows → Экран → Графика → Стандартные параметры графики.\nПолезно на RTX 30xx+ и RX 6000+.");
            ui.end_row();
        });

        ui.add_space(12.0);

        // ── Parasite killer ───────────────────────────────────────────────────
        section_header(ui, "🐛 Убийца паразитных процессов", "🔫");
        ui.label(RichText::new(
            "Останавливает ненужные сервисы Windows которые жрут CPU/RAM во время игры.\nВсё восстанавливается кнопкой «Откатить всё»."
        ).color(C_TEXT_DIM).size(12.0));
        ui.add_space(6.0);

        egui::Grid::new("parasites").num_columns(2).spacing([20.0, 7.0]).show(ui, |ui| {
            ui.horizontal(|ui| {
                checkbox_tip(ui, &mut app.config.system_tweaks.kill_superfetch,
                    "Отключить SysMain (Superfetch)",
                    "Superfetch предзагружает данные в RAM — мешает играм занимать память.\nОстановка даёт +200–500 MB свободной RAM.");
                badge(ui, "+RAM", C_SUCCESS);
            });
            ui.horizontal(|ui| {
                checkbox_tip(ui, &mut app.config.system_tweaks.kill_xbox_services,
                    "Отключить Xbox сервисы",
                    "XboxGipSvc, XblGameSave, XblAuthManager — полностью бесполезны если нет Xbox.\nЭкономит CPU и RAM.");
                badge(ui, "+CPU", C_SUCCESS);
            });
            ui.end_row();

            ui.horizontal(|ui| {
                checkbox_tip(ui, &mut app.config.system_tweaks.kill_windows_search,
                    "Отключить Windows Search",
                    "WSearch постоянно индексирует файлы — нагружает диск и CPU во время игры.\n+5–15% диска освобождается.");
                badge(ui, "+DISK", C_SUCCESS);
            });
            ui.horizontal(|ui| {
                checkbox_tip(ui, &mut app.config.system_tweaks.kill_print_spooler,
                    "Отключить Print Spooler",
                    "Сервис печати — абсолютно бесполезен во время игры.\nЕсли нет принтера — отключай смело.");
            });
            ui.end_row();

            checkbox_tip(ui, &mut app.config.system_tweaks.kill_fax,
                "Отключить Fax",
                "Сервис факса — никто не использует. Просто занимает место.");
            checkbox_tip(ui, &mut app.config.system_tweaks.disable_telemetry,
                "Отключить телеметрию Windows",
                "Телеметрия отправляет данные Microsoft в фоне.\nОтключение снижает фоновую сетевую активность.");
            ui.end_row();

            checkbox_tip(ui, &mut app.config.system_tweaks.disable_tips,
                "Отключить советы и уведомления",
                "Советы Windows, реклама в меню Пуск, подсказки — всё это прерывает игру.");
            ui.end_row();
        });

        ui.add_space(12.0);

        // ── Network optimization ──────────────────────────────────────────────
        section_header(ui, "Сеть (меньше пинг в Rust)", "🌐");
        egui::Grid::new("net_tweaks").num_columns(2).spacing([20.0, 7.0]).show(ui, |ui| {
            ui.horizontal(|ui| {
                checkbox_tip(ui, &mut app.config.system_tweaks.disable_nagle,
                    "Отключить алгоритм Нейгла (Nagle)",
                    "Алгоритм Нейгла буферизует сетевые пакеты = задержки.\nОтключение даёт -5–20ms пинга в онлайн играх!");
                badge(ui, "-ПИНГ", C_INFO);
            });
            ui.horizontal(|ui| {
                checkbox_tip(ui, &mut app.config.system_tweaks.optimize_network,
                    "Оптимизировать TCP параметры",
                    "TcpAckFrequency=1, TCPNoDelay=1, DefaultTTL=64.\nСтандартные игровые настройки TCP стека.");
                badge(ui, "-ПИНГ", C_INFO);
            });
            ui.end_row();
        });

        // ── Hardware info ─────────────────────────────────────────────────────
        ui.add_space(12.0);
        section_header(ui, "Ваше железо", "💻");
        egui::Frame::none()
            .fill(C_METAL)
            .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                let (tier, tc) = crate::core::hardware::get_hardware_tier(&hw);
                info_row(ui, "CPU", &format!("{} ({} cores)", hw.cpu_name, hw.cpu_cores), None);
                info_row(ui, "GPU", &hw.gpu_name, None);
                info_row(ui, "RAM", &format!("{} GB ({} MB)", hw.ram_total_mb / 1024, hw.ram_total_mb), None);
                info_row(ui, "OS",  &hw.os_version, None);
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Tier:").color(C_TEXT_DIM).size(13.0));
                    ui.label(RichText::new(tier).color(tc).size(13.0).strong());
                });
            });

        // NVIDIA tips
        if hw.is_nvidia {
            ui.add_space(10.0);
            section_header(ui, "NVIDIA Control Panel", "🟢");
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

        // AMD tips
        if hw.is_amd {
            ui.add_space(10.0);
            section_header(ui, "AMD Software", "🔴");
            egui::Frame::none()
                .fill(C_METAL)
                .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
                .rounding(egui::Rounding::same(5.0))
                .show(ui, |ui| {
                    for tip in &[
                        "📌 Anti-Lag → Enable (+снижение инпут-лага)",
                        "📌 Radeon Image Sharpening → On",
                        "📌 FSR 2/3 → Quality или Balanced",
                        "📌 Enhanced Sync вместо VSync",
                        "📌 EXPO/XMP в BIOS для Ryzen!",
                    ] {
                        ui.label(RichText::new(*tip).size(12.0).color(C_TEXT));
                    }
                });
        }

        // Cached system status
        ui.add_space(10.0);
        section_header(ui, "Статус сервисов (кэш)", "📊");
        egui::Frame::none()
            .fill(C_METAL)
            .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                for (key, val) in app.cached_perf_info.clone() {
                    let color = if val.contains("✅") { Some(C_SUCCESS) }
                                else if val.contains("⚠️") { Some(C_WARNING) }
                                else { None };
                    info_row(ui, &key, &val, color);
                }
                ui.add_space(4.0);
                if secondary_button(ui, "🔄 Обновить статус").clicked() {
                    app.pending_action = Some(crate::app::PendingAction::RefreshSysInfo);
                }
            });

        // ── Buttons ───────────────────────────────────────────────────────────
        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if action_button(ui, "⚡  Применить выбранное").clicked() {
                app.pending_action = Some(crate::app::PendingAction::ApplyTweaks);
            }
            ui.add_space(8.0);
            if secondary_button(ui, "🔄 Откатить всё").clicked() {
                app.pending_action = Some(crate::app::PendingAction::RevertTweaks);
            }
        });
        if tweaks_applied {
            ui.add_space(4.0);
            ui.label(RichText::new("✅ Твики применены").color(C_SUCCESS).size(12.0));
        }
        ui.add_space(8.0);
    });
}
