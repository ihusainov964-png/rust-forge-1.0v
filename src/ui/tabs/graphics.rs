// ui/tabs/graphics.rs — Graphics Settings tab (egui 0.22)

use egui::{RichText, Ui};
use crate::app::RustForgeApp;
use crate::ui::theme::*;
use crate::ui::widgets::*;

pub fn draw_graphics_tab(app: &mut RustForgeApp, ui: &mut Ui) {

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(4.0);

        egui::Frame::none()
            .fill(C_INFO.linear_multiply(0.08))
            .stroke(egui::Stroke::new(1.0, C_INFO.linear_multiply(0.4)))
            .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                ui.label(RichText::new(
                    "ℹ️  Настройки записываются в client.cfg и применяются при следующем запуске Rust.\nНажми 'Применить конфиг' или используй кнопку Запустить Rust."
                ).color(C_TEXT_DIM).size(12.0));
            });
        ui.add_space(10.0);

        // Master Quality
        section_header(ui, "Качество графики", "🎨");
        ui.horizontal(|ui| {
            ui.label(RichText::new("Graphics Quality").color(C_TEXT).size(13.0));
            let mut v = app.config.graphics.quality as f32;
            ui.add(egui::Slider::new(&mut v, 0.0f32..=6.0).integer().step_by(1.0))
                .on_hover_text("Главный ползунок. 0–1 = MAX FPS, 2–3 = Баланс, 4–6 = Красота.\nДроп с 6→2 даёт +30–50 FPS на mid-range железе!");
            app.config.graphics.quality = v as u8;
            let (lbl, col) = match app.config.graphics.quality {
                0 => ("POTATO", C_ERROR),
                1 => ("MAX FPS", C_RUST_BRIGHT),
                2 => ("PVP", C_WARNING),
                3 => ("BALANCED", C_WARNING),
                4 => ("HIGH", C_SUCCESS),
                5 => ("ULTRA", C_SUCCESS),
                _ => ("MAX", C_INFO),
            };
            ui.label(RichText::new(lbl).color(col).size(13.0).strong());
        });
        ui.add_space(6.0);

        // Shadows
        section_header(ui, "Тени  —  огромное влияние на FPS!", "🌑");
        ui.horizontal(|ui| {
            ui.label(RichText::new("Shadow Quality").color(C_TEXT).size(13.0));
            let mut v = app.config.graphics.shadow_quality as f32;
            ui.add(egui::Slider::new(&mut v, 0.0f32..=3.0).integer())
                .on_hover_text("0 = OFF (+15–30 FPS!), 1 = Low, 2 = Med, 3 = High\nВместе с Cascades = до -38 FPS!");
            app.config.graphics.shadow_quality = v as u8;
        });
        checkbox_tip(ui, &mut app.config.graphics.shadow_cascades, "Shadow Cascades",
            "Каскадные тени. OFF = +10–20 FPS дополнительно.");
        ui.add_space(6.0);

        // Water
        section_header(ui, "Вода  —  до -27 FPS!", "💧");
        ui.horizontal(|ui| {
            ui.label(RichText::new("Water Quality").color(C_TEXT).size(13.0));
            let mut v = app.config.graphics.water_quality as f32;
            ui.add(egui::Slider::new(&mut v, 0.0f32..=3.0).integer())
                .on_hover_text("0 = MAX FPS. Нет конкурентного недостатка при отключении.");
            app.config.graphics.water_quality = v as u8;
        });
        checkbox_tip(ui, &mut app.config.graphics.water_reflections, "Water Reflections",
            "Отражения воды. OFF = +10–15 FPS.");
        ui.add_space(6.0);

        // Draw distance & LOD
        section_header(ui, "Дистанция прорисовки и LOD", "🏔️");
        labeled_slider_f32(ui, "Draw Distance", &mut app.config.graphics.draw_distance, 500.0..=3000.0, "m",
            "Оптимум: 1500–2000. Больше 2000 = всего +5% видимости за -7 FPS.\nМеньше 1500 = теряешь угрозы на дальних дистанциях.");
        labeled_slider_f32(ui, "LOD Bias", &mut app.config.graphics.lod_bias, 0.25..=3.0, "",
            "Низкий LOD Bias у баз = до +50% FPS! Рекомендуется 0.5–1.0 для PVP.");
        ui.add_space(6.0);

        // AA & Post FX
        section_header(ui, "Сглаживание и постобработка", "🔲");
        ui.horizontal(|ui| {
            ui.label(RichText::new("Anti-Aliasing").color(C_TEXT).size(13.0));
            egui::ComboBox::from_id_source("aa_combo")
                .selected_text(match app.config.graphics.anti_aliasing {
                    0 => "OFF", 1 => "FXAA", 2 => "SMAA", 3 => "TAA", _ => "OFF",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.config.graphics.anti_aliasing, 0, "OFF (Max FPS)");
                    ui.selectable_value(&mut app.config.graphics.anti_aliasing, 1, "FXAA (Fast)");
                    ui.selectable_value(&mut app.config.graphics.anti_aliasing, 2, "SMAA (PVP рекомендация)");
                    ui.selectable_value(&mut app.config.graphics.anti_aliasing, 3, "TAA (Quality)");
                });
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("Anisotropic Filtering").color(C_TEXT).size(13.0));
            egui::ComboBox::from_id_source("af_combo")
                .selected_text(match app.config.graphics.anisotropic {
                    0 => "OFF", 1 => "2x", 2 => "4x", 3 => "8x", 4 => "16x", _ => "4x",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app.config.graphics.anisotropic, 0, "OFF");
                    ui.selectable_value(&mut app.config.graphics.anisotropic, 1, "2x");
                    ui.selectable_value(&mut app.config.graphics.anisotropic, 2, "4x (рекомендуется)");
                    ui.selectable_value(&mut app.config.graphics.anisotropic, 3, "8x");
                    ui.selectable_value(&mut app.config.graphics.anisotropic, 4, "16x");
                });
        });
        ui.add_space(4.0);
        egui::Grid::new("postfx").num_columns(3).spacing([20.0, 5.0]).show(ui, |ui| {
            checkbox_tip(ui, &mut app.config.graphics.motion_blur, "Motion Blur",
                "ОТКЛЮЧИ! Не даёт преимущества, только мешает в PVP.");
            checkbox_tip(ui, &mut app.config.graphics.depth_of_field, "Depth of Field",
                "ОТКЛЮЧИ! Размывает экран, ухудшает видимость.");
            checkbox_tip(ui, &mut app.config.graphics.bloom, "Bloom", "Свечение. Отключи для чистоты картинки.");
            ui.end_row();
            checkbox_tip(ui, &mut app.config.graphics.lens_dirt, "Lens Dirt", "Грязь на линзе. Чисто косметика.");
            ui.end_row();
        });
        ui.add_space(6.0);

        // World quality
        section_header(ui, "Качество мира", "🌿");
        egui::Grid::new("world_qual").num_columns(2).spacing([30.0, 6.0]).show(ui, |ui| {
            labeled_slider_u8(ui, "Grass Quality", &mut app.config.graphics.grass_quality, 0, 3,
                "Трава. 0 = OFF (заметный буст FPS на слабых PC).");
            labeled_slider_u8(ui, "Tree Quality",  &mut app.config.graphics.tree_quality,  0, 3, "Деревья.");
            ui.end_row();
            labeled_slider_u8(ui, "Rock Quality",  &mut app.config.graphics.rock_quality,  0, 3, "Камни.");
            labeled_slider_u8(ui, "Particles",     &mut app.config.graphics.particle_quality, 0, 3,
                "Частицы (огонь, дым, взрывы).");
            ui.end_row();
        });
        ui.add_space(6.0);

        // Gameplay visuals
        section_header(ui, "Игровые настройки", "👤");
        egui::Grid::new("gameplay_vis").num_columns(3).spacing([20.0, 5.0]).show(ui, |ui| {
            checkbox_tip(ui, &mut app.config.graphics.eye_blink,    "Eye Blink",  "Мигание глаз.");
            checkbox_tip(ui, &mut app.config.graphics.eye_movement, "Eye Movement", "Движение глаз.");
            checkbox_tip(ui, &mut app.config.graphics.blood,        "Blood Effects",
                "Кровь при ударах. Отключи для чистоты экрана в бою.");
            ui.end_row();
            checkbox_tip(ui, &mut app.config.graphics.headbob, "Head Bob",
                "Покачивание камеры. Большинство PVP игроков отключают.");
            checkbox_tip(ui, &mut app.config.graphics.vm_fov_scale, "VM FOV Scale",
                "Масштабирование FOV вьюмодели.");
            ui.end_row();
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("FOV").size(13.0));
            let mut v = app.config.graphics.fov as f32;
            ui.add(egui::Slider::new(&mut v, 60.0f32..=120.0).integer().suffix("°"))
                .on_hover_text("Field of View. PVP стандарт: 90°.");
            app.config.graphics.fov = v as u8;
        });

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if action_button(ui, "💾  Применить конфиг (client.cfg)").clicked() {
                app.pending_action = Some(crate::app::PendingAction::WriteConfig);
            }
            ui.add_space(8.0);
            if secondary_button(ui, "🔄 Сбросить").clicked() {
                app.config.graphics = crate::config::GraphicsConfig::default();
                app.push_notification("Графика сброшена", crate::config::NotificationKind::Info);
            }
        });
    });
}
