// ui/tabs/advanced.rs — 40 hidden FPS-eating convar toggles (egui 0.22)
// Legal only: client convars written to client.cfg, nothing touches EAC.

use egui::{RichText, Ui};
use crate::app::RustForgeApp;
use crate::config::AdvancedTweaks;
use crate::ui::theme::*;
use crate::ui::widgets::*;

/// A checkbox whose ON position means "disabled for FPS" — inverted display
/// over the underlying `enabled` flag, so ticking the box = feature OFF.
fn disable_toggle(ui: &mut Ui, enabled_flag: &mut bool, label: &str, tip: &str) {
    let mut disabled = !*enabled_flag;
    if checkbox_tip(ui, &mut disabled, label, tip).changed() {
        *enabled_flag = !disabled;
    }
}

pub fn draw_advanced_tab(app: &mut RustForgeApp, ui: &mut Ui) {
    let disabled = app.config.advanced.disabled_count();

    egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        ui.add_space(4.0);

        egui::Frame::none()
            .fill(C_INFO.linear_multiply(0.08))
            .stroke(egui::Stroke::new(1.0, C_INFO.linear_multiply(0.4)))
            .inner_margin(egui::style::Margin::symmetric(12.0, 8.0))
            .rounding(egui::Rounding::same(5.0))
            .show(ui, |ui| {
                ui.label(RichText::new(
                    "ℹ️  40 «паразитных» client-convar'ов Rust, которых нет в обычном меню настроек.\n\
                     ✅ Галочка = эффект ВЫКЛЮЧЕН (больше FPS). Пустой чекбокс = эффект включён (как в ванильной игре).\n\
                     Всё это graphics.*/effects.*/audio.* — пишется в client.cfg, ничего не трогает EAC \
                     и server-convar'ы. После апдейта игры конкретный convar иногда переименовывают — \
                     если эффект пропал, набери в игре F1 → find <слово>, чтобы проверить актуальное имя."
                ).color(C_TEXT_DIM).size(12.0));
            });
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("Отключено: {} / {}", disabled, AdvancedTweaks::TOTAL))
                .color(if disabled > 0 { C_SUCCESS } else { C_TEXT_DIM }).size(13.0).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if secondary_button(ui, "Включить все").clicked() {
                    let custom = app.config.advanced.custom_convars.clone();
                    app.config.advanced = AdvancedTweaks::all_on();
                    app.config.advanced.custom_convars = custom;
                    app.push_notification("Все эффекты включены", crate::config::NotificationKind::Info);
                }
                if action_button(ui, "🔥 Максимальный FPS (выключить все)").clicked() {
                    let custom = app.config.advanced.custom_convars.clone();
                    app.config.advanced = AdvancedTweaks::all_off();
                    app.config.advanced.custom_convars = custom;
                    app.push_notification("Все эффекты отключены", crate::config::NotificationKind::Success);
                }
            });
        });
        ui.add_space(10.0);

        section_header(ui, "Мир и окружение", "🌫️");
        egui::Grid::new("adv_world").num_columns(2).spacing([24.0, 6.0]).show(ui, |ui| {
            disable_toggle(ui, &mut app.config.advanced.fog_detail, "🚫 Детализация тумана",
                "graphics.fog — плотный туман считается каждый кадр. OFF на дальних дистанциях = чище FPS.");
            disable_toggle(ui, &mut app.config.advanced.cloud_detail, "🚫 Детализация облаков",
                "graphics.clouds — объёмные облака в небе, почти всегда вне поля зрения в PVP.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.wind_simulation, "🚫 Симуляция ветра",
                "wind.enabled — влияет на деревья/траву/флаги, считается для всей загруженной зоны.");
            disable_toggle(ui, &mut app.config.advanced.dust_particles, "🚫 Пылевые частицы",
                "effects.dust — частицы пыли в воздухе на открытых картах и в карьерах.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.ambient_occlusion, "🚫 Ambient Occlusion",
                "graphics.ao — мягкие затенения в углах и щелях. Дорогой пост-эффект.");
            disable_toggle(ui, &mut app.config.advanced.sun_shafts, "🚫 Лучи солнца (God Rays)",
                "graphics.sunshafts — объёмные лучи света. Красиво, но не бесплатно.");
            ui.end_row();
        });
        ui.add_space(6.0);

        section_header(ui, "Экранные пост-эффекты", "🖥️");
        egui::Grid::new("adv_screen").num_columns(2).spacing([24.0, 6.0]).show(ui, |ui| {
            disable_toggle(ui, &mut app.config.advanced.vignette, "🚫 Виньетирование",
                "graphics.vignette — затемнение краёв экрана, чисто косметика.");
            disable_toggle(ui, &mut app.config.advanced.chromatic_aberration, "🚫 Хроматическая аберрация",
                "graphics.chromaticaberration — цветные разводы по краям. Мешает целиться.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.film_grain, "🚫 Зерно плёнки (Film Grain)",
                "graphics.filmgrain — шум поверх картинки, постоянный пост-проход.");
            disable_toggle(ui, &mut app.config.advanced.lens_flare, "🚫 Блики объектива (Lens Flare)",
                "graphics.lensflare — блики от солнца/фонарей, лишний рендер-пасс.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.screen_space_reflections, "🚫 Screen Space Reflections",
                "graphics.ssr — отражения на мокрых поверхностях. Один из самых тяжёлых эффектов.");
            disable_toggle(ui, &mut app.config.advanced.color_grading_lut, "🚫 Цветокоррекция (LUT)",
                "graphics.colorgrading — финальный цветовой пост-проход поверх кадра.");
            ui.end_row();
        });
        ui.add_space(6.0);

        section_header(ui, "Тени и детализация", "🌑");
        egui::Grid::new("adv_shadow").num_columns(2).spacing([24.0, 6.0]).show(ui, |ui| {
            disable_toggle(ui, &mut app.config.advanced.contact_shadows, "🚫 Контактные тени",
                "graphics.contactshadows — мягкие тени под мелкими объектами. Свободный FPS при отключении.");
            disable_toggle(ui, &mut app.config.advanced.mesh_shadow_distance, "🚫 Тени от мешей на дистанции",
                "mesh.shadowdistance — тени объектов на дальних дистанциях. Агрессивно, но даёт много FPS.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.shader_lod_full, "🚫 Полная детализация шейдеров",
                "graphics.shaderlod — снижение LOD шейдеров на удалении экономит GPU.");
            disable_toggle(ui, &mut app.config.advanced.terrain_detail_recalc, "🚫 Пересчёт детализации террейна",
                "terrain.idleinterval — как часто пересчитывается детализация ландшафта, пока стоишь.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.grass_displacement, "🚫 Прогибание травы",
                "grass.displacement — трава физически прогибается при ходьбе. Минус нагрузка на CPU.");
            ui.end_row();
        });
        ui.add_space(6.0);

        section_header(ui, "Частицы и декали", "💥");
        egui::Grid::new("adv_particles").num_columns(2).spacing([24.0, 6.0]).show(ui, |ui| {
            disable_toggle(ui, &mut app.config.advanced.blood_decals, "🚫 Декали крови",
                "effects.blooddecals — пятна крови остаются на текстурах и накапливаются в бою.");
            disable_toggle(ui, &mut app.config.advanced.bullet_impact_decals, "🚫 Декали от пуль",
                "effects.bulletdecals — следы от попаданий на стенах и земле.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.explosion_particles, "🚫 Частицы взрывов",
                "effects.explosionparticles — детализация облака взрыва при рейдах.");
            disable_toggle(ui, &mut app.config.advanced.muzzle_flash_particles, "🚫 Частицы дульной вспышки",
                "effects.muzzleflash — детализированная вспышка выстрела вместо упрощённой.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.footstep_dust, "🚫 Пыль от шагов",
                "effects.footstepdust — частицы пыли под ногами при беге по земле.");
            disable_toggle(ui, &mut app.config.advanced.fire_smoke_particles, "🚫 Дым от костров/пожаров",
                "effects.firesmoke — объёмный дым от костров, печей, горящих построек.");
            ui.end_row();
        });
        ui.add_space(6.0);

        section_header(ui, "Физика объектов и мира", "🧍");
        egui::Grid::new("adv_physics").num_columns(2).spacing([24.0, 6.0]).show(ui, |ui| {
            disable_toggle(ui, &mut app.config.advanced.ragdoll_physics_detail, "🚫 Детальная физика рэгдоллов",
                "ragdoll.detail — количество физических костей у трупов после смерти.");
            disable_toggle(ui, &mut app.config.advanced.corpse_extended_lifetime, "🚫 Долгое время жизни трупов",
                "corpse.extendedlifetime — трупы дольше остаются на карте и считаются в физике.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.cloth_physics, "🚫 Физика одежды",
                "graphics.clothphysics — покачивание плащей/тряпок на персонажах.");
            disable_toggle(ui, &mut app.config.advanced.foliage_wind_sway, "🚫 Покачивание листвы на ветру",
                "graphics.foliagesway — анимация качания деревьев и кустов.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.deployable_shadow_detail, "🚫 Тени построек/декора",
                "graphics.deployableshadows — детальные тени от мебели и построек внутри баз.");
            ui.end_row();
        });
        ui.add_space(6.0);

        section_header(ui, "Звук и амбиент", "🔊");
        egui::Grid::new("adv_audio").num_columns(2).spacing([24.0, 6.0]).show(ui, |ui| {
            disable_toggle(ui, &mut app.config.advanced.ambience_layer, "🚫 Амбиентный звуковой слой",
                "ambience.enabled — фоновая звуковая подложка окружения (ветер, гул, шумы).");
            disable_toggle(ui, &mut app.config.advanced.wind_audio, "🚫 Звук ветра",
                "audio.wind — динамический звук ветра, пересчитывается по погоде.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.wildlife_audio, "🚫 Звуки живности",
                "audio.wildlife — звуки животных и птиц по всей загруженной зоне.");
            disable_toggle(ui, &mut app.config.advanced.spatial_voice_processing, "🚫 Пространственная обработка голоса",
                "audio.spatialvoice — 3D-позиционирование голосового чата других игроков.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.footstep_reverb, "🚫 Реверберация шагов",
                "audio.footstepreverb — эхо шагов в помещениях/пещерах, лишняя аудио-обработка.");
            ui.end_row();
        });
        ui.add_space(6.0);

        section_header(ui, "Рендер и разное", "⚙️");
        egui::Grid::new("adv_render").num_columns(2).spacing([24.0, 6.0]).show(ui, |ui| {
            disable_toggle(ui, &mut app.config.advanced.water_simulation_detail, "🚫 Детализация симуляции воды",
                "water.simdetail — волны и симуляция поверхности воды в реальном времени.");
            disable_toggle(ui, &mut app.config.advanced.super_sampling, "🚫 Супер-сэмплинг (SSAA)",
                "gfx.ssaa — самый дорогой вид сглаживания, рендерит кадр в повышенном разрешении.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.post_process_aa, "🚫 Пост-процессное сглаживание",
                "effects.aa_quality — дополнительный AA-проход поверх основного сглаживания.");
            disable_toggle(ui, &mut app.config.advanced.prop_pooling_optimization, "🚫 Пул объектов окружения",
                "graphics.proppooling — держит в памяти доп. объекты для быстрого спавна декора.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.camera_shake, "🚫 Тряска камеры",
                "graphics.camerashake — тряска экрана при взрывах и попаданиях. Мешает прицеливаться.");
            disable_toggle(ui, &mut app.config.advanced.combat_text_popups, "🚫 Всплывающий текст урона",
                "ui.combattext — цифры урона над целью, лишняя перерисовка UI в бою.");
            ui.end_row();
            disable_toggle(ui, &mut app.config.advanced.loot_glow_effects, "🚫 Свечение лута",
                "effects.lootglow — контурная подсветка предметов на земле.");
            disable_toggle(ui, &mut app.config.advanced.aggressive_shadow_lod, "🚫 Агрессивный LOD теней",
                "graphics.aggressiveShadowLod — подтверждён в официальных патчнотах Facepunch (июль 2026), \
                 снижает детализацию теней на удалении сильнее обычного.");
            ui.end_row();
        });

        ui.add_space(10.0);
        section_header(ui, "Свои convar'ы (найденные через F1 → find)", "📝");
        ui.label(RichText::new(
            "Игра обновляется каждый месяц, и я не могу гарантированно знать все новые convar'ы. \
             Нашёл в игре что-то новое, чего нет выше? Впиши сюда, по одной команде на строку \
             (например: graphics.something 0) — приложение допишет их в client.cfg вместе с остальными."
        ).color(C_TEXT_DIM).size(12.0));
        ui.add_space(4.0);
        ui.add(
            egui::TextEdit::multiline(&mut app.config.advanced.custom_convars)
                .desired_rows(4)
                .desired_width(f32::INFINITY)
                .hint_text("graphics.example 0\neffects.example 1")
        );

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            if action_button(ui, "💾  Применить конфиг (client.cfg)").clicked() {
                app.pending_action = Some(crate::app::PendingAction::WriteConfig);
            }
            ui.add_space(8.0);
            if secondary_button(ui, "🔄 Сбросить").clicked() {
                app.config.advanced = AdvancedTweaks::default();
                app.push_notification("Расширенные настройки сброшены", crate::config::NotificationKind::Info);
            }
        });
    });
}
