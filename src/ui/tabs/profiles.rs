// ui/tabs/profiles.rs — Profiles tab (egui 0.22)

use egui::{RichText, Ui};
use crate::app::RustForgeApp;
use crate::ui::theme::*;
use crate::ui::widgets::*;

pub fn draw_profiles_tab(app: &mut RustForgeApp, ui: &mut Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(4.0);
        section_header(ui, "Профили настроек", "📁");
        ui.label(RichText::new("Выбери готовый профиль или сохрани свои настройки.").color(C_TEXT_DIM).size(12.0));
        ui.add_space(8.0);

        let active = app.config.active_profile.clone();
        for name in &["Max FPS", "Balanced PVP", "High Visibility", "Custom"] {
            if !app.config.profiles.contains_key(*name) { continue; }
            let profile = app.config.profiles[*name].clone();
            let is_active = active == *name;

            egui::Frame::none()
                .fill(if is_active { C_RUST_DARK.linear_multiply(0.3) } else { C_METAL })
                .stroke(egui::Stroke::new(
                    if is_active { 2.0 } else { 1.0 },
                    if is_active { C_RUST } else { C_METAL_MID },
                ))
                .inner_margin(egui::style::Margin::symmetric(14.0, 10.0))
                .rounding(egui::Rounding::same(6.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(profile.icon).size(24.0));
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&profile.name).color(C_TEXT_BRIGHT).size(15.0).strong());
                                if is_active { ui.add_space(8.0); badge(ui, "АКТИВЕН", C_RUST); }
                            });
                            ui.label(RichText::new(&profile.description).color(C_TEXT_DIM).size(12.0));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if !is_active && secondary_button(ui, "Применить").clicked() {
                                app.config.launch_options = profile.launch_options.clone();
                                app.config.graphics       = profile.graphics.clone();
                                app.config.active_profile = name.to_string();
                                app.push_notification(
                                    &format!("Профиль '{}' применён!", name),
                                    crate::config::NotificationKind::Success,
                                );
                            }
                        });
                    });
                });
            ui.add_space(6.0);
        }

        ui.add_space(10.0);
        section_header(ui, "Сохранить текущее как профиль", "💾");
        ui.horizontal(|ui| {
            ui.label(RichText::new("Имя:").color(C_TEXT_DIM).size(13.0));
            ui.add(egui::TextEdit::singleline(&mut app.new_profile_name)
                .desired_width(200.0)
                .hint_text("Мой профиль"));
            if secondary_button(ui, "Сохранить").clicked() {
                let name = app.new_profile_name.trim().to_string();
                if !name.is_empty() {
                    app.config.profiles.insert(name.clone(), crate::config::Profile {
                        name: name.clone(),
                        description: "Пользовательский профиль".to_string(),
                        icon: "🔧".to_string(),
                        launch_options: app.config.launch_options.clone(),
                        graphics: app.config.graphics.clone(),
                    });
                    app.config.active_profile = name.clone();
                    app.new_profile_name.clear();
                    app.push_notification(
                        &format!("Профиль '{}' сохранён!", name),
                        crate::config::NotificationKind::Success,
                    );
                }
            }
        });

        ui.add_space(16.0);
        section_header(ui, "Резервная копия", "🗄️");
        ui.label(RichText::new(
            "RustForge автоматически создаёт бэкап client.cfg и localconfig.vdf перед любыми изменениями."
        ).color(C_TEXT_DIM).size(12.0));
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            if secondary_button(ui, "🔄 Восстановить оригинал").clicked() {
                app.pending_action = Some(crate::app::PendingAction::RestoreBackup);
            }
            ui.add_space(8.0);
            if secondary_button(ui, "📂 Открыть папку конфигов").clicked() {
                if let Some(path) = crate::core::steam::get_rust_config_path() {
                    open_path(&path);
                }
            }
        });
    });
}

fn open_path(path: &std::path::Path) {
    #[cfg(target_os = "windows")]
    { let _ = std::process::Command::new("explorer").arg(path).spawn(); }
    #[cfg(target_os = "linux")]
    { let _ = std::process::Command::new("xdg-open").arg(path).spawn(); }
}
