// ui/tabs/launch.rs — Launch Options tab (egui 0.22)

use egui::{RichText, Ui};
use crate::app::RustForgeApp;
use crate::ui::theme::*;
use crate::ui::widgets::*;

pub fn draw_launch_tab(app: &mut RustForgeApp, ui: &mut Ui) {
    let ram = app.config.detected_hardware.ram_total_mb;

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(4.0);

        // Preview bar
        let preview = app.config.launch_options.build_string(ram);
        egui::Frame::none()
            .fill(C_METAL)
            .stroke(egui::Stroke::new(1.0, C_RUST_DARK))
            .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("▶ LAUNCH OPTIONS:").color(C_TEXT_DIM).size(11.0).monospace());
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(if preview.is_empty() { "(none)" } else { &preview })
                            .color(C_RUST_BRIGHT).size(12.0).monospace(),
                    );
                });
            });
        ui.add_space(10.0);

        // Performance
        section_header(ui, "Производительность", "⚡");
        egui::Grid::new("lo_perf").num_columns(2).spacing([24.0, 6.0]).show(ui, |ui| {
            checkbox_tip(ui, &mut app.config.launch_options.high_priority, "-high",
                "-high: запускает игру с высоким приоритетом процесса.\n+2-5 FPS на загруженных системах.");
            checkbox_tip(ui, &mut app.config.launch_options.no_log, "-nolog",
                "Отключает запись логов. Меньше нагрузка на диск.");
            ui.end_row();
            checkbox_tip(ui, &mut app.config.launch_options.window_exclusive, "-window-mode exclusive",
                "Эксклюзивный fullscreen: меньше латентность, больше FPS.\nМожет усложнить Alt+Tab.");
            checkbox_tip(ui, &mut app.config.launch_options.malloc_system, "-malloc=system",
                "Системный аллокатор вместо Unity. Помогает при утечках памяти.");
            ui.end_row();
            checkbox_tip(ui, &mut app.config.launch_options.force_d3d11, "-force-d3d11-no-singlethreaded",
                "Отключает однопоточный D3D11. Может дать +FPS на многоядерных CPU.");
            checkbox_tip(ui, &mut app.config.launch_options.force_feature_level, "-force-feature-level-11-0",
                "Принудительно DX11 Feature Level 11.0. Помогает на некоторых GPU.");
            ui.end_row();
            checkbox_tip(ui, &mut app.config.launch_options.no_steam_controller, "-nosteamcontroller",
                "Отключает Steam Controller API. Ускоряет запуск.");
            ui.end_row();
        });

        ui.add_space(8.0);
        section_header(ui, "Память", "🧠");
        ui.horizontal(|ui| {
            checkbox_tip(ui, &mut app.config.launch_options.max_mem_auto, "Авто -maxMem",
                "Устанавливает -maxMem = 75% вашей RAM.\nПредотвращает OutOfMemory краши.");
            if app.config.launch_options.max_mem_auto && ram > 0 {
                let mm = ((ram * 3 / 4) as u64).min(32768);
                ui.label(RichText::new(format!("→ -maxMem={}", mm)).color(C_RUST_BRIGHT).monospace().size(12.0));
            }
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("-gc.buffer").color(C_TEXT_DIM).size(13.0));
            ui.add(egui::Slider::new(&mut app.config.launch_options.gc_buffer, 256..=4096).step_by(256.0).suffix(" MB"))
                .on_hover_text("Unity GC буфер. Больше = реже GC паузы = меньше статтеров.\nРекомендуется: 1024–2048 MB");
        });

        ui.add_space(8.0);
        section_header(ui, "Графика (Launch Options)", "🎮");
        ui.horizontal(|ui| {
            ui.label(RichText::new("-headlerp").color(C_TEXT_DIM).size(13.0));
            ui.add(egui::Slider::new(&mut app.config.launch_options.headlerp, 0u32..=200).suffix(""))
                .on_hover_text("Head Bob lerp speed. 100 = стандарт, выше = меньше качки.");
        });
        ui.add_space(4.0);
        egui::Grid::new("lo_gfx").num_columns(2).spacing([24.0, 6.0]).show(ui, |ui| {
            checkbox_tip(ui, &mut app.config.launch_options.disable_waves, "-graphics.waves 0",
                "Отключает волны воды. +5–15 FPS на береговых локациях.");
            checkbox_tip(ui, &mut app.config.launch_options.disable_gibs, "-effects.maxgibs -1",
                "Отключает физику дебриса. +3–8 FPS в активных боях.");
            ui.end_row();
        });

        ui.add_space(8.0);
        section_header(ui, "Кастомные опции", "✏️");
        ui.label(RichText::new("Дополнительные launch options:").color(C_TEXT_DIM).size(12.0));
        ui.add_space(3.0);
        ui.add(
            egui::TextEdit::singleline(&mut app.config.launch_options.custom_options)
                .hint_text("-myoption value ...")
                .desired_width(f32::INFINITY)
                .font(egui::FontId::monospace(13.0)),
        ).on_hover_text("Добавится в конец строки launch options");

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if action_button(ui, "🚀  ЗАПУСТИТЬ RUST").clicked() {
                app.pending_action = Some(crate::app::PendingAction::LaunchRust);
            }
            ui.add_space(8.0);
            if secondary_button(ui, "📋 Копировать строку").clicked() {
                let s = app.config.launch_options.build_string(ram);
                ui.output_mut(|o| o.copied_text = s);
                app.push_notification("Launch options скопированы!", crate::config::NotificationKind::Success);
            }
            ui.add_space(8.0);
            if secondary_button(ui, "🔄 Сбросить").clicked() {
                app.config.launch_options = crate::config::LaunchOptions::default();
                app.push_notification("Launch options сброшены", crate::config::NotificationKind::Info);
            }
        });
    });
}
