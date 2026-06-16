// ui/tabs/system.rs — Full system optimizer with animations (egui 0.22)

use egui::{Color32, RichText, Ui};
use crate::app::RustForgeApp;
use crate::ui::theme::*;
use crate::ui::widgets::*;

// ── Animated toggle button ────────────────────────────────────────────────────
fn toggle_switch(ui: &mut Ui, on: &mut bool, label: &str, tip: &str) -> bool {
    let size  = egui::vec2(44.0, 22.0);
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());

    // Animate position
    let t = ui.ctx().animate_bool(resp.id, *on);
    let bg = if *on { C_RUST.linear_multiply(0.9 + t * 0.1) } else { C_METAL_LIGHT };
    let circle_x = rect.left() + 11.0 + t * 22.0;

    ui.painter().rect_filled(rect, egui::Rounding::same(11.0), bg);
    ui.painter().circle_filled(
        egui::pos2(circle_x, rect.center().y),
        9.0,
        Color32::WHITE,
    );

    // Border
    ui.painter().rect_stroke(rect, egui::Rounding::same(11.0),
        egui::Stroke::new(1.0, if *on { C_RUST } else { C_METAL_MID }));

    let r = ui.label(RichText::new(label).size(13.0).color(if *on { C_TEXT_BRIGHT } else { C_TEXT_DIM }));
    let resp = resp.union(r);
    if !tip.is_empty() { resp.clone().on_hover_text(tip); }

    if resp.clicked() {
        *on = !*on;
        ui.ctx().request_repaint();
        return true;
    }
    false
}

// ── Category card ─────────────────────────────────────────────────────────────
fn category_card<R>(
    ui: &mut Ui,
    id: &str,
    icon: &str,
    title: &str,
    subtitle: &str,
    badge_text: Option<(&str, Color32)>,
    content: impl FnOnce(&mut Ui) -> R,
) {
    egui::Frame::none()
        .fill(C_METAL)
        .stroke(egui::Stroke::new(1.0, C_METAL_MID))
        .inner_margin(egui::style::Margin::symmetric(14.0, 10.0))
        .rounding(egui::Rounding::same(8.0))
        .show(ui, |ui| {
            // Header
            ui.horizontal(|ui| {
                ui.label(RichText::new(icon).size(20.0));
                ui.add_space(6.0);
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(title).size(14.0).color(C_TEXT_BRIGHT).strong());
                        if let Some((bt, bc)) = badge_text {
                            ui.add_space(6.0);
                            badge(ui, bt, bc);
                        }
                    });
                    ui.label(RichText::new(subtitle).size(11.0).color(C_TEXT_DIM));
                });
            });
            ui.add(egui::Separator::default().spacing(6.0));
            ui.add_space(4.0);
            content(ui);
        });
    ui.add_space(8.0);
}

// ── Toggle row ────────────────────────────────────────────────────────────────
fn toggle_row(ui: &mut Ui, on: &mut bool, label: &str, desc: &str, impact: &str, impact_color: Color32) {
    ui.horizontal(|ui| {
        // Toggle
        let size = egui::vec2(40.0, 20.0);
        let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());
        let t = ui.ctx().animate_bool(resp.id, *on);
        let bg = C_RUST.linear_multiply(0.4 + t * 0.6);
        ui.painter().rect_filled(rect, egui::Rounding::same(10.0), if *on { bg } else { C_METAL_LIGHT });
        ui.painter().circle_filled(
            egui::pos2(rect.left() + 10.0 + t * 20.0, rect.center().y),
            8.0, Color32::WHITE,
        );
        ui.painter().rect_stroke(rect, egui::Rounding::same(10.0),
            egui::Stroke::new(1.0, if *on { C_RUST } else { C_METAL_MID }));
        if resp.clicked() { *on = !*on; ui.ctx().request_repaint(); }

        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(label).size(13.0).color(if *on { C_TEXT_BRIGHT } else { C_TEXT }));
                ui.add_space(6.0);
                ui.label(RichText::new(impact).size(11.0).color(impact_color).strong());
            });
            ui.label(RichText::new(desc).size(11.0).color(C_TEXT_DIM));
        });
    });
    ui.add_space(4.0);
}

// ── Main draw function ────────────────────────────────────────────────────────
pub fn draw_system_tab(app: &mut RustForgeApp, ui: &mut Ui) {
    let hw = app.config.detected_hardware.clone();
    let tweaks_applied = app.config.system_tweaks.tweaks_applied;

    egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        ui.add_space(4.0);

        // ── Top hardware summary ──────────────────────────────────────────────
        egui::Frame::none()
            .fill(C_METAL_DARK)
            .stroke(egui::Stroke::new(1.5, C_RUST_DARK))
            .inner_margin(egui::style::Margin::symmetric(16.0, 10.0))
            .rounding(egui::Rounding::same(8.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let (tier, tc) = crate::core::hardware::get_hardware_tier(&hw);

                    // Tier badge (animated pulse)
                    let pulse = (ui.input(|i| i.time).sin() * 0.15 + 0.85) as f32;
                    ui.vertical_centered(|ui| {
                        ui.add_space(4.0);
                        ui.label(RichText::new(tier).size(16.0).color(tc.linear_multiply(pulse)).strong());
                        ui.label(RichText::new("СИСТЕМА").size(9.0).color(C_TEXT_DIM));
                    });

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(16.0);

                    // HW grid
                    egui::Grid::new("hw_grid").num_columns(2).spacing([24.0, 3.0]).show(ui, |ui| {
                        ui.label(RichText::new("CPU").color(C_TEXT_DIM).size(11.0));
                        ui.label(RichText::new(format!("{} ({} ядер)", hw.cpu_name, hw.cpu_cores)).size(12.0).color(C_TEXT));
                        ui.end_row();
                        ui.label(RichText::new("GPU").color(C_TEXT_DIM).size(11.0));
                        ui.label(RichText::new(&hw.gpu_name).size(12.0).color(C_TEXT));
                        ui.end_row();
                        ui.label(RichText::new("RAM").color(C_TEXT_DIM).size(11.0));
                        ui.label(RichText::new(format!("{} GB", hw.ram_total_mb / 1024)).size(12.0).color(C_TEXT));
                        ui.end_row();
                        ui.label(RichText::new("OS").color(C_TEXT_DIM).size(11.0));
                        ui.label(RichText::new(&hw.os_version).size(11.0).color(C_TEXT_DIM));
                        ui.end_row();
                    });

                    // GPU vendor badges
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if hw.is_nvidia { badge(ui, "NVIDIA", Color32::from_rgb(118, 185, 0)); }
                        if hw.is_amd   { badge(ui, "AMD", Color32::from_rgb(237, 28, 36)); }
                    });
                });
            });
        ui.add_space(12.0);

        // Request repaint for pulse animation
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(500));

        // ── PERFORMANCE ───────────────────────────────────────────────────────
        category_card(ui, "perf", "⚡", "Производительность Windows",
            "Разгони систему для максимального FPS", Some(("КРИТИЧНО", C_RUST)), |ui| {
            toggle_row(ui, &mut app.config.system_tweaks.ultimate_power_plan,
                "Ultimate Performance Power Plan",
                "Убирает ограничения CPU clock между кадрами → CPU работает на 100% всегда",
                "+5–15% CPU", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.game_mode,
                "Windows Game Mode",
                "Windows приоритизирует игровой процесс над всеми фоновыми задачами",
                "+FPS", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.disable_xbox_game_bar,
                "Отключить Xbox Game Bar",
                "Game Bar перехватывает Win+G, работает в фоне и жрёт RAM даже когда не нужен",
                "-RAM", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.set_high_timer_res,
                "Системный таймер 1ms (timeBeginPeriod)",
                "Точный таймер = плавный фреймтайм, меньше микростатттеров между кадрами",
                "Smooth", C_INFO);
            toggle_row(ui, &mut app.config.system_tweaks.hardware_gpu_scheduling,
                "HAGS — подсказка",
                "Настраивается вручную: Параметры → Экран → Графика. Полезно RTX30xx+ / RX6000+",
                "INFO", C_TEXT_DIM);
            toggle_row(ui, &mut app.config.system_tweaks.disable_fullscreen_optimizations,
                "Fullscreen Optimizations — подсказка",
                "ПКМ на RustClient.exe → Свойства → Совместимость → галочка",
                "INFO", C_TEXT_DIM);
        });

        // ── PARASITE KILLER ───────────────────────────────────────────────────
        category_card(ui, "parasites", "🐛", "Убийца паразитных процессов",
            "Останавливает сервисы которые жрут ресурсы во время игры", Some(("ГОРЯЧО", C_WARNING)), |ui| {
            toggle_row(ui, &mut app.config.system_tweaks.kill_superfetch,
                "SysMain / Superfetch",
                "Предзагружает файлы в RAM впрок → мешает игре занять память. Полностью бесполезен для геймеров",
                "+500MB RAM", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.kill_xbox_services,
                "Xbox сервисы (XboxGipSvc, XblGameSave, XblAuthManager)",
                "3 сервиса работают всегда даже без Xbox. Совершенно бесполезны если нет Xbox-контроллера",
                "+CPU/RAM", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.kill_windows_search,
                "Windows Search (WSearch)",
                "Постоянно индексирует файлы на диске → 100% загрузка диска именно во время загрузки игры",
                "+DISK", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.kill_print_spooler,
                "Print Spooler (Spooler)",
                "Сервис управления печатью. Если нет принтера — абсолютно бесполезен",
                "+CPU", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.kill_fax,
                "Fax Service",
                "Сервис факса. 2026 год. Никто не использует факс.",
                "+RAM", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.kill_remote_registry,
                "Remote Registry",
                "Разрешает удалённый доступ к реестру — бесполезно и небезопасно для обычного пользователя",
                "+SEC", Color32::from_rgb(100, 180, 255));
            toggle_row(ui, &mut app.config.system_tweaks.kill_tablet_input,
                "Tablet Input Service",
                "Сервис для планшетного ввода и рукописного ввода. Бесполезен без планшета",
                "+RAM", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.kill_secondary_logon,
                "Secondary Logon",
                "Запуск программ от имени другого пользователя — не нужно большинству игроков",
                "+SEC", Color32::from_rgb(100, 180, 255));
            toggle_row(ui, &mut app.config.system_tweaks.kill_diagnostic_policy,
                "Diagnostic Policy Service",
                "Диагностика сети и системы в фоне — постоянно что-то проверяет и пишет логи",
                "+DISK", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.kill_downloaded_maps,
                "Downloaded Maps Manager",
                "Сервис карт Windows — автоматически обновляет карты. 100% бесполезен",
                "+NET", C_SUCCESS);
        });

        // ── NETWORK ───────────────────────────────────────────────────────────
        category_card(ui, "network", "🌐", "Оптимизация сети",
            "Меньше пинг, стабильнее соединение в Rust", Some(("-ПИНГ", C_INFO)), |ui| {
            toggle_row(ui, &mut app.config.system_tweaks.disable_nagle,
                "Отключить алгоритм Нейгла (Nagle Algorithm)",
                "Наглевский алгоритм накапливает мелкие пакеты → задержка отправки. Отключение даёт моментальную отправку",
                "-5–20ms ПИНГ", C_INFO);
            toggle_row(ui, &mut app.config.system_tweaks.optimize_network,
                "Оптимизировать TCP параметры",
                "TcpAckFrequency=1, TCPNoDelay=1, DefaultTTL=64 — стандартные игровые настройки",
                "-ПИНГ", C_INFO);
            toggle_row(ui, &mut app.config.system_tweaks.disable_qos,
                "Отключить QoS резервирование пропускной способности",
                "Windows резервирует 20% канала для системных нужд. Отключение отдаёт весь канал игре",
                "+20% КАНАЛ", C_INFO);
            toggle_row(ui, &mut app.config.system_tweaks.optimize_dns,
                "Ускорить DNS кэш (DnsCacheTimeout)",
                "Увеличивает время кэширования DNS ответов — меньше DNS запросов во время игры",
                "-ЛАГ", C_INFO);
            toggle_row(ui, &mut app.config.system_tweaks.disable_network_throttling,
                "Отключить Network Throttling Index",
                "Windows намеренно замедляет сетевой трафик для мультимедиа. Отключи для игр",
                "+СКОРОСТЬ", C_INFO);
        });

        // ── PRIVACY / TELEMETRY ───────────────────────────────────────────────
        category_card(ui, "privacy", "🔒", "Конфиденциальность и телеметрия",
            "Останови фоновую слежку которая грузит сеть и диск", None, |ui| {
            toggle_row(ui, &mut app.config.system_tweaks.disable_telemetry,
                "Телеметрия Windows (AllowTelemetry=0)",
                "Windows постоянно отправляет данные об использовании на серверы Microsoft — нагружает сеть",
                "-СЕТЬ", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.disable_tips,
                "Советы, реклама и уведомления",
                "Советы в меню Пуск, реклама приложений, подсказки — прерывают игру всплывашками",
                "-МУСОР", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.disable_activity_history,
                "История активности (Timeline)",
                "Windows записывает всё что ты делаешь в Timeline — постоянная запись на диск",
                "-DISK", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.disable_location,
                "Служба геолокации",
                "Если не используешь геолокацию в приложениях — отключи. Меньше фоновой активности",
                "+RAM", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.disable_cortana,
                "Cortana",
                "Голосовой помощник Windows — работает в фоне, слушает микрофон, жрёт RAM",
                "+RAM", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.disable_compat_telemetry,
                "Compatibility Telemetry (CompatTelRunner)",
                "Процесс совместимости приложений — регулярно запускается и грузит CPU/диск",
                "+CPU", C_SUCCESS);
        });

        // ── VISUAL ────────────────────────────────────────────────────────────
        category_card(ui, "visual", "🎭", "Визуальные эффекты Windows",
            "Отключи красоту ради скорости — Windows станет быстрее", None, |ui| {
            toggle_row(ui, &mut app.config.system_tweaks.disable_animations,
                "Отключить анимации Windows",
                "Анимации открытия окон, сворачивания, появления меню — всё это жрёт GPU в фоне",
                "+GPU", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.disable_transparency,
                "Отключить прозрачность интерфейса",
                "Эффект Acrylic / матовое стекло в taskbar — постоянно нагружает GPU даже в игре",
                "+GPU", C_SUCCESS);
            toggle_row(ui, &mut app.config.system_tweaks.classic_menu,
                "Классическое контекстное меню (Win11)",
                "Возвращает старое быстрое меню ПКМ вместо нового медленного в Windows 11",
                "+SPEED", C_INFO);
        });

        // ── GPU / Drivers ─────────────────────────────────────────────────────
        if hw.is_nvidia || hw.is_amd {
            let (gpu_title, gpu_sub, tips) = if hw.is_nvidia {
                ("🟢 NVIDIA Control Panel", "Настройки для максимального FPS и минимального лага",
                 vec![
                    ("Low Latency Mode", "Ultra — минимальная задержка рендера"),
                    ("Power Management Mode", "Prefer Maximum Performance"),
                    ("Texture Filtering Quality", "High Performance"),
                    ("Vertical Sync", "Off — используй лимит FPS в игре"),
                    ("Shader Cache Size", "Unlimited"),
                    ("Reflex Low Latency", "On + Boost (RTX серия)"),
                    ("DLSS", "Quality или Balanced для RTX"),
                    ("OpenGL rendering GPU", "Выбери свою карту явно"),
                 ])
            } else {
                ("🔴 AMD Software: Adrenalin", "Настройки для Rust на AMD GPU",
                 vec![
                    ("Anti-Lag", "Enable — снижение инпут-лага"),
                    ("Radeon Image Sharpening", "On — чёткость без потери FPS"),
                    ("FSR 2/3", "Quality или Balanced в Rust"),
                    ("Enhanced Sync", "Вместо VSync — меньше тиринга"),
                    ("EXPO/XMP в BIOS", "Критично для Ryzen систем!"),
                    ("Chill", "Off — не нужен в Rust"),
                 ])
            };

            category_card(ui, "gpu", &gpu_title[..2], &gpu_title[2..], gpu_sub, None, |ui| {
                for (setting, desc) in tips {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("▸").color(C_RUST).size(13.0));
                        ui.add_space(4.0);
                        ui.vertical(|ui| {
                            ui.label(RichText::new(setting).size(13.0).color(C_TEXT_BRIGHT));
                            ui.label(RichText::new(desc).size(11.0).color(C_TEXT_DIM));
                        });
                    });
                    ui.add_space(3.0);
                }
            });
        }

        // ── Status panel ──────────────────────────────────────────────────────
        category_card(ui, "status", "📊", "Статус сервисов",
            "Текущее состояние системы (обновляется по кнопке)", None, |ui| {
            for (key, val) in app.cached_perf_info.clone() {
                let vc = if val.contains("✅") { C_SUCCESS }
                         else if val.contains("⚠️") { C_WARNING }
                         else { C_TEXT_DIM };
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("{:<20}", key)).size(12.0).color(C_TEXT_DIM));
                    ui.label(RichText::new(&val).size(12.0).color(vc));
                });
            }
            ui.add_space(6.0);
            if secondary_button(ui, "🔄 Обновить статус").clicked() {
                app.pending_action = Some(crate::app::PendingAction::RefreshSysInfo);
            }
        });

        // ── Action buttons ────────────────────────────────────────────────────
        ui.separator();
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            let apply_btn = egui::Button::new(
                RichText::new("⚡  ПРИМЕНИТЬ ВСЁ ВЫБРАННОЕ").size(15.0).strong().color(Color32::WHITE)
            )
            .fill(C_RUST)
            .stroke(egui::Stroke::new(2.0, C_RUST_BRIGHT))
            .min_size(egui::vec2(220.0, 38.0));

            if ui.add(apply_btn).clicked() {
                app.pending_action = Some(crate::app::PendingAction::ApplyTweaks);
            }
            ui.add_space(10.0);
            if secondary_button(ui, "🔄 Откатить всё").clicked() {
                app.pending_action = Some(crate::app::PendingAction::RevertTweaks);
            }
        });
        if tweaks_applied {
            ui.add_space(4.0);
            egui::Frame::none()
                .fill(C_SUCCESS.linear_multiply(0.15))
                .stroke(egui::Stroke::new(1.0, C_SUCCESS.linear_multiply(0.5)))
                .inner_margin(egui::style::Margin::symmetric(10.0, 5.0))
                .rounding(egui::Rounding::same(4.0))
                .show(ui, |ui| {
                    ui.label(RichText::new("✅ Твики применены — перезапусти Rust для эффекта").color(C_SUCCESS).size(12.0));
                });
        }
        ui.add_space(12.0);
    });
}
