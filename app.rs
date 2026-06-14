// app.rs — Main application state and eframe update loop (egui 0.22)
// Performance: all heavy ops cached, no blocking calls in UI thread

use crate::config::{AppConfig, Notification, NotificationKind};
use crate::core::{
    hardware::detect_hardware,
    persistence::{load_config, save_config},
    steam::{launch_rust_via_steam, restore_launch_options_backup, write_rust_config},
    tweaks::{apply_system_tweaks, revert_system_tweaks},
};
use crate::ui::{
    apply_theme,
    tabs::{draw_about_tab, draw_graphics_tab, draw_launch_tab, draw_profiles_tab, draw_system_tab},
    widgets::{action_button, render_notifications, secondary_button},
    theme::*,
};
use egui::{Color32, RichText};
use log::{error, info};

#[derive(Debug, Clone, PartialEq)]
pub enum PendingAction {
    LaunchRust,
    WriteConfig,
    ApplyTweaks,
    RevertTweaks,
    RestoreBackup,
    ClearCache,
    RefreshSysInfo,
}

pub struct RustForgeApp {
    pub config: AppConfig,
    pub notifications: Vec<Notification>,
    pub pending_action: Option<PendingAction>,
    pub action_results: Vec<String>,
    pub show_results_modal: bool,
    pub new_profile_name: String,
    pub steam_found: bool,

    // Cached system info — refreshed only on demand, not every frame
    pub cached_perf_info: Vec<(String, String)>,
    pub perf_info_age: std::time::Instant,

    // Save throttle
    last_save: std::time::Instant,
}

impl RustForgeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        apply_theme(&cc.egui_ctx);

        // Heavy ops only once at startup
        let mut config = load_config();
        config.detected_hardware = detect_hardware();
        info!(
            "Hardware: {} cores, {} MB RAM, GPU: {}",
            config.detected_hardware.cpu_cores,
            config.detected_hardware.ram_total_mb,
            config.detected_hardware.gpu_name,
        );

        let steam_found = crate::core::steam::find_steam_path().is_some();

        // Cache initial perf info
        let cached_perf_info = crate::core::tweaks::get_windows_perf_info();

        Self {
            config,
            notifications: Vec::new(),
            pending_action: None,
            action_results: Vec::new(),
            show_results_modal: false,
            new_profile_name: String::new(),
            steam_found,
            cached_perf_info,
            perf_info_age: std::time::Instant::now(),
            last_save: std::time::Instant::now(),
        }
    }

    pub fn push_notification(&mut self, msg: &str, kind: NotificationKind) {
        self.notifications.push(Notification {
            message: msg.to_string(),
            kind,
            timestamp: std::time::Instant::now(),
        });
        info!("[notif] {}", msg);
    }

    /// Get cached perf info — auto-refreshes every 10 seconds
    pub fn get_perf_info(&mut self) -> &Vec<(String, String)> {
        if self.perf_info_age.elapsed().as_secs() > 10 {
            self.cached_perf_info = crate::core::tweaks::get_windows_perf_info();
            self.perf_info_age = std::time::Instant::now();
        }
        &self.cached_perf_info
    }

    fn process_pending_action(&mut self) {
        let action = match self.pending_action.take() {
            Some(a) => a,
            None => return,
        };

        match action {
            PendingAction::LaunchRust => {
                if !self.steam_found {
                    self.push_notification("Steam не найден!", NotificationKind::Error);
                    return;
                }
                let cmds = self.config.graphics.to_console_commands();
                if let Err(e) = write_rust_config(&cmds) {
                    error!("Config write failed: {}", e);
                }
                let ram  = self.config.detected_hardware.ram_total_mb;
                let opts = self.config.launch_options.build_string(ram);
                match launch_rust_via_steam(&opts) {
                    Ok(_)  => self.push_notification("🚀 Rust запущен!", NotificationKind::Success),
                    Err(e) => self.push_notification(&format!("Ошибка запуска: {}", e), NotificationKind::Error),
                }
            }

            PendingAction::WriteConfig => {
                let cmds = self.config.graphics.to_console_commands();
                let ram  = self.config.detected_hardware.ram_total_mb;
                let opts = self.config.launch_options.build_string(ram);
                let mut results = Vec::new();
                match write_rust_config(&cmds) {
                    Ok(p)  => results.push(format!("✅ client.cfg записан: {:?}", p)),
                    Err(e) => results.push(format!("❌ client.cfg: {}", e)),
                }
                match crate::core::steam::apply_steam_launch_options(&opts) {
                    Ok(_)  => results.push(format!("✅ Launch options: {}", opts)),
                    Err(e) => results.push(format!("⚠️ Launch options: {}", e)),
                }
                self.action_results    = results;
                self.show_results_modal = true;
                self.push_notification("Конфиг записан!", NotificationKind::Success);
            }

            PendingAction::ApplyTweaks => {
                match apply_system_tweaks(&self.config.system_tweaks) {
                    Ok(results) => {
                        self.config.system_tweaks.tweaks_applied = true;
                        self.action_results    = results;
                        self.show_results_modal = true;
                        self.push_notification("Твики применены!", NotificationKind::Success);
                        // Refresh perf info after applying tweaks
                        self.cached_perf_info = crate::core::tweaks::get_windows_perf_info();
                        self.perf_info_age = std::time::Instant::now();
                    }
                    Err(e) => self.push_notification(&format!("Ошибка твиков: {}", e), NotificationKind::Error),
                }
            }

            PendingAction::RevertTweaks => {
                match revert_system_tweaks() {
                    Ok(results) => {
                        self.config.system_tweaks.tweaks_applied = false;
                        self.action_results    = results;
                        self.show_results_modal = true;
                        self.push_notification("Настройки откачены!", NotificationKind::Info);
                        self.cached_perf_info = crate::core::tweaks::get_windows_perf_info();
                        self.perf_info_age = std::time::Instant::now();
                    }
                    Err(e) => self.push_notification(&format!("Ошибка отката: {}", e), NotificationKind::Error),
                }
            }

            PendingAction::RestoreBackup => {
                match restore_launch_options_backup() {
                    Ok(_)  => self.push_notification("Бэкап восстановлен!", NotificationKind::Success),
                    Err(e) => self.push_notification(&format!("Бэкап: {}", e), NotificationKind::Error),
                }
                if let Some(cfg_path) = crate::core::steam::get_rust_config_path() {
                    let backup   = cfg_path.join("client.cfg.rfbackup");
                    let original = cfg_path.join("client.cfg");
                    if backup.exists() {
                        if let Err(e) = std::fs::copy(&backup, &original) {
                            self.push_notification(&format!("client.cfg restore: {}", e), NotificationKind::Warning);
                        }
                    }
                }
            }

            PendingAction::ClearCache => {
                match crate::core::steam::clear_rust_cache() {
                    Ok(n)  => self.push_notification(&format!("Очищено {} файлов кэша", n), NotificationKind::Success),
                    Err(e) => self.push_notification(&format!("Ошибка очистки: {}", e), NotificationKind::Error),
                }
            }

            PendingAction::RefreshSysInfo => {
                self.cached_perf_info = crate::core::tweaks::get_windows_perf_info();
                self.perf_info_age = std::time::Instant::now();
                self.push_notification("Статус системы обновлён", NotificationKind::Info);
            }
        }

        // Throttled save — not more than once per 2 seconds
        if self.last_save.elapsed().as_secs() >= 2 {
            if let Err(e) = save_config(&self.config) {
                error!("Failed to save config: {}", e);
            }
            self.last_save = std::time::Instant::now();
        }
    }
}

impl eframe::App for RustForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_pending_action();

        // Auto-save every 30 seconds (not every frame!)
        if self.last_save.elapsed().as_secs() >= 30 {
            let _ = save_config(&self.config);
            self.last_save = std::time::Instant::now();
        }

        // ── Top bar ──────────────────────────────────────────────────────────
        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::none()
                .fill(C_METAL_DARK)
                .stroke(egui::Stroke::new(1.0, C_METAL_MID))
                .inner_margin(egui::style::Margin::symmetric(16.0, 10.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("⚙").size(26.0).color(C_RUST));
                    ui.add_space(4.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new("RUSTFORGE").size(18.0).color(C_RUST_BRIGHT).strong());
                        ui.label(RichText::new("Ultimate Game Optimizer").size(10.0).color(C_TEXT_DIM));
                    });
                    ui.add_space(20.0);

                    if self.steam_found {
                        ui.label(RichText::new("🟢 Steam найден").size(11.0).color(C_SUCCESS));
                    } else {
                        ui.label(RichText::new("🔴 Steam не найден").size(11.0).color(C_ERROR));
                    }
                    ui.add_space(12.0);

                    let hw = &self.config.detected_hardware;
                    if !hw.gpu_name.is_empty() && hw.gpu_name != "Unknown GPU" {
                        let short = hw.gpu_name
                            .replace("NVIDIA GeForce ", "")
                            .replace("AMD Radeon ", "");
                        let short = if short.len() > 22 { &short[..22] } else { &short };
                        ui.label(RichText::new(format!("GPU: {}", short)).size(11.0).color(C_TEXT_DIM));
                    }
                    if hw.ram_total_mb > 0 {
                        ui.label(RichText::new(
                            format!("RAM: {} GB", hw.ram_total_mb / 1024)
                        ).size(11.0).color(C_TEXT_DIM));
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let boost_btn = egui::Button::new(
                            RichText::new("⚡ ONE-CLICK BOOST").size(14.0).strong().color(Color32::WHITE),
                        )
                        .fill(C_RUST)
                        .stroke(egui::Stroke::new(2.0, C_RUST_BRIGHT))
                        .min_size(egui::vec2(170.0, 34.0));

                        if ui.add(boost_btn)
                            .on_hover_text("Применяет профиль «Balanced PVP» + запускает Rust")
                            .clicked()
                        {
                            if let Some(p) = self.config.profiles.get("Balanced PVP") {
                                self.config.launch_options = p.launch_options.clone();
                                self.config.graphics       = p.graphics.clone();
                                self.config.active_profile = "Balanced PVP".to_string();
                            }
                            self.pending_action = Some(PendingAction::LaunchRust);
                            self.push_notification("⚡ ONE-CLICK BOOST активирован!", NotificationKind::Success);
                        }
                    });
                });
            });

        // ── Bottom bar ───────────────────────────────────────────────────────
        egui::TopBottomPanel::bottom("bottom_panel")
            .frame(egui::Frame::none()
                .fill(C_METAL_DARK)
                .stroke(egui::Stroke::new(1.0, C_METAL_MID))
                .inner_margin(egui::style::Margin::symmetric(16.0, 6.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("RustForge v1.0.0").size(11.0).color(C_TEXT_DIM));
                    ui.separator();
                    ui.label(RichText::new(
                        "Только легальные твики • Безопасно для EAC"
                    ).size(11.0).color(C_TEXT_DIM));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if secondary_button(ui, "🗑️ Очистить кэш").clicked() {
                            self.pending_action = Some(PendingAction::ClearCache);
                        }
                    });
                });
            });

        // ── Left sidebar ─────────────────────────────────────────────────────
        egui::SidePanel::left("tabs_panel")
            .resizable(false)
            .exact_width(155.0)
            .frame(egui::Frame::none()
                .fill(C_METAL)
                .stroke(egui::Stroke::new(1.0, C_METAL_MID))
                .inner_margin(egui::style::Margin::symmetric(8.0, 12.0)))
            .show(ctx, |ui| {
                ui.add_space(4.0);
                for (i, (icon, label)) in [
                    ("🚀", "Запуск"),
                    ("🎨", "Графика"),
                    ("⚙️", "Система"),
                    ("📁", "Профили"),
                    ("ℹ️", "О программе"),
                ].iter().enumerate() {
                    let is_active = self.config.window_state.active_tab == i;
                    let btn = egui::Button::new(
                        RichText::new(format!("{} {}", icon, label))
                            .size(13.5)
                            .color(if is_active { C_RUST_BRIGHT } else { C_TEXT }),
                    )
                    .fill(if is_active { C_RUST_DARK } else { egui::Color32::TRANSPARENT })
                    .stroke(egui::Stroke::new(if is_active { 1.5 } else { 0.0 }, C_RUST))
                    .min_size(egui::vec2(136.0, 36.0));

                    if ui.add(btn).clicked() {
                        self.config.window_state.active_tab = i;
                    }
                    ui.add_space(4.0);
                }

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);
                let hw = &self.config.detected_hardware;
                if hw.ram_total_mb > 0 {
                    ui.label(RichText::new(format!("{} GB RAM", hw.ram_total_mb / 1024))
                        .size(11.0).color(C_TEXT_DIM));
                }
                if hw.cpu_cores > 0 {
                    ui.label(RichText::new(format!("{} CPU cores", hw.cpu_cores))
                        .size(11.0).color(C_TEXT_DIM));
                }
                let (tier, tc) = crate::core::hardware::get_hardware_tier(hw);
                ui.label(RichText::new(tier).size(11.0).color(tc).strong());
            });

        // ── Main content ─────────────────────────────────────────────────────
        egui::CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(C_METAL_DARK)
                .inner_margin(egui::style::Margin::symmetric(16.0, 12.0)))
            .show(ctx, |ui| {
                render_notifications(ui, &mut self.notifications);

                match self.config.window_state.active_tab {
                    0 => draw_launch_tab(self, ui),
                    1 => draw_graphics_tab(self, ui),
                    2 => draw_system_tab(self, ui),
                    3 => draw_profiles_tab(self, ui),
                    4 => draw_about_tab(self, ui),
                    _ => {}
                }
            });

        // ── Results modal ────────────────────────────────────────────────────
        if self.show_results_modal {
            let results = self.action_results.clone();
            egui::Window::new("Результаты")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .frame(egui::Frame::window(&ctx.style())
                    .fill(C_METAL)
                    .stroke(egui::Stroke::new(1.5, C_RUST)))
                .show(ctx, |ui| {
                    ui.add_space(4.0);
                    for r in &results {
                        ui.label(RichText::new(r).size(13.0));
                    }
                    ui.add_space(8.0);
                    if action_button(ui, "Закрыть").clicked() {
                        self.show_results_modal = false;
                        self.action_results.clear();
                    }
                });
        }

        // Repaint ONLY when notifications are active — otherwise egui is idle
        if !self.notifications.is_empty() {
            ctx.request_repaint_after(std::time::Duration::from_millis(150));
        }
        // No unconditional repaint = zero CPU when idle!
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Err(e) = save_config(&self.config) {
            error!("Save on exit failed: {}", e);
        }
        info!("RustForge exiting.");
    }
}
