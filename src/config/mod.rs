// config/mod.rs — All configuration data structures with serde serialization

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete app state saved to disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub launch_options: LaunchOptions,
    pub graphics: GraphicsConfig,
    pub system_tweaks: SystemTweaks,
    pub profiles: HashMap<String, Profile>,
    pub active_profile: String,
    pub backup_created: bool,
    pub first_run: bool,
    pub window_state: WindowState,
    pub detected_hardware: HardwareInfo,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            launch_options: LaunchOptions::default(),
            graphics: GraphicsConfig::default(),
            system_tweaks: SystemTweaks::default(),
            profiles: Self::default_profiles(),
            active_profile: "Custom".to_string(),
            backup_created: false,
            first_run: true,
            window_state: WindowState::default(),
            detected_hardware: HardwareInfo::default(),
        }
    }
}

impl AppConfig {
    fn default_profiles() -> HashMap<String, Profile> {
        let mut map = HashMap::new();

        map.insert("Max FPS".to_string(), Profile {
            name: "Max FPS".to_string(),
            description: "Максимальный FPS — минимальное качество".to_string(),
            icon: "🚀".to_string(),
            launch_options: LaunchOptions {
                high_priority: true,
                no_log: true,
                window_exclusive: true,
                max_mem_auto: true,
                gc_buffer: 2048,
                headlerp: 100,
                disable_waves: true,
                disable_gibs: true,
                force_d3d11: false,
                force_feature_level: true,
                malloc_system: true,
                no_steam_controller: false,
                custom_options: String::new(),
            },
            graphics: GraphicsConfig {
                quality: 0,
                shadow_quality: 0,
                shadow_cascades: false,
                water_quality: 0,
                water_reflections: false,
                lod_bias: 0.5,
                draw_distance: 1500.0,
                anisotropic: 0,
                anti_aliasing: 0,
                motion_blur: false,
                depth_of_field: false,
                bloom: false,
                lens_dirt: false,
                eye_blink: false,
                eye_movement: false,
                blood: false,
                headbob: false,
                fov: 90,
                vm_fov_scale: true,
                grass_quality: 0,
                tree_quality: 0,
                rock_quality: 0,
                particle_quality: 0,
                decor_quality: 0,
            },
        });

        map.insert("Balanced PVP".to_string(), Profile {
            name: "Balanced PVP".to_string(),
            description: "Баланс FPS и видимости — для PVP".to_string(),
            icon: "⚔️".to_string(),
            launch_options: LaunchOptions {
                high_priority: true,
                no_log: true,
                window_exclusive: false,
                max_mem_auto: true,
                gc_buffer: 1024,
                headlerp: 100,
                disable_waves: true,
                disable_gibs: true,
                force_d3d11: false,
                force_feature_level: false,
                malloc_system: false,
                no_steam_controller: false,
                custom_options: String::new(),
            },
            graphics: GraphicsConfig {
                quality: 1,
                shadow_quality: 0,
                shadow_cascades: false,
                water_quality: 0,
                water_reflections: false,
                lod_bias: 1.0,
                draw_distance: 2000.0,
                anisotropic: 2,
                anti_aliasing: 1,
                motion_blur: false,
                depth_of_field: false,
                bloom: false,
                lens_dirt: false,
                eye_blink: false,
                eye_movement: false,
                blood: false,
                headbob: false,
                fov: 90,
                vm_fov_scale: true,
                grass_quality: 1,
                tree_quality: 1,
                rock_quality: 1,
                particle_quality: 1,
                decor_quality: 1,
            },
        });

        map.insert("High Visibility".to_string(), Profile {
            name: "High Visibility".to_string(),
            description: "Максимальная видимость врагов и объектов".to_string(),
            icon: "👁️".to_string(),
            launch_options: LaunchOptions {
                high_priority: true,
                no_log: true,
                window_exclusive: false,
                max_mem_auto: true,
                gc_buffer: 1024,
                headlerp: 100,
                disable_waves: true,
                disable_gibs: true,
                force_d3d11: false,
                force_feature_level: false,
                malloc_system: false,
                no_steam_controller: false,
                custom_options: String::new(),
            },
            graphics: GraphicsConfig {
                quality: 2,
                shadow_quality: 0,
                shadow_cascades: false,
                water_quality: 1,
                water_reflections: false,
                lod_bias: 2.0,
                draw_distance: 2500.0,
                anisotropic: 4,
                anti_aliasing: 1,
                motion_blur: false,
                depth_of_field: false,
                bloom: true,
                lens_dirt: false,
                eye_blink: false,
                eye_movement: false,
                blood: false,
                headbob: false,
                fov: 90,
                vm_fov_scale: true,
                grass_quality: 2,
                tree_quality: 2,
                rock_quality: 2,
                particle_quality: 1,
                decor_quality: 2,
            },
        });

        map.insert("Custom".to_string(), Profile {
            name: "Custom".to_string(),
            description: "Ваши персональные настройки".to_string(),
            icon: "🔧".to_string(),
            launch_options: LaunchOptions::default(),
            graphics: GraphicsConfig::default(),
        });

        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchOptions {
    pub high_priority: bool,
    pub no_log: bool,
    pub window_exclusive: bool,
    pub max_mem_auto: bool,
    pub gc_buffer: u32,
    pub headlerp: u32,
    pub disable_waves: bool,
    pub disable_gibs: bool,
    pub force_d3d11: bool,
    pub force_feature_level: bool,
    pub malloc_system: bool,
    pub no_steam_controller: bool,
    pub custom_options: String,
}

impl Default for LaunchOptions {
    fn default() -> Self {
        Self {
            high_priority: true,
            no_log: false,
            window_exclusive: false,
            max_mem_auto: true,
            gc_buffer: 1024,
            headlerp: 100,
            disable_waves: true,
            disable_gibs: true,
            force_d3d11: false,
            force_feature_level: false,
            malloc_system: false,
            no_steam_controller: false,
            custom_options: String::new(),
        }
    }
}

impl LaunchOptions {
    /// Builds the final launch options string
    pub fn build_string(&self, detected_ram_mb: u64) -> String {
        let mut opts = Vec::new();

        if self.high_priority    { opts.push("-high".to_string()); }
        if self.no_log           { opts.push("-nolog".to_string()); }
        if self.window_exclusive { opts.push("-window-mode exclusive".to_string()); }
        if self.malloc_system    { opts.push("-malloc=system".to_string()); }
        if self.force_d3d11      { opts.push("-force-d3d11-no-singlethreaded".to_string()); }
        if self.force_feature_level { opts.push("-force-feature-level-11-0".to_string()); }
        if self.no_steam_controller { opts.push("-nosteamcontroller".to_string()); }

        if self.max_mem_auto && detected_ram_mb > 0 {
            // Use 75% of detected RAM, capped at 32768 MB
            let max_mem = ((detected_ram_mb * 3 / 4) as u64).min(32768);
            opts.push(format!("-maxMem={}", max_mem));
        }

        if self.gc_buffer > 0 {
            opts.push(format!("-gc.buffer {}", self.gc_buffer));
        }

        if self.headlerp > 0 {
            opts.push(format!("-headlerp {}", self.headlerp));
        }

        if self.disable_waves  { opts.push("-graphics.waves 0".to_string()); }
        if self.disable_gibs   { opts.push("-effects.maxgibs -1".to_string()); }

        if !self.custom_options.trim().is_empty() {
            opts.push(self.custom_options.trim().to_string());
        }

        opts.join(" ")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsConfig {
    pub quality: u8,           // 0-6
    pub shadow_quality: u8,    // 0-3
    pub shadow_cascades: bool,
    pub water_quality: u8,     // 0-3
    pub water_reflections: bool,
    pub lod_bias: f32,         // 0.25 - 3.0
    pub draw_distance: f32,    // 500 - 3000
    pub anisotropic: u8,       // 0,2,4,8,16
    pub anti_aliasing: u8,     // 0=off, 1=FXAA, 2=SMAA, 3=TAA
    pub motion_blur: bool,
    pub depth_of_field: bool,
    pub bloom: bool,
    pub lens_dirt: bool,
    pub eye_blink: bool,
    pub eye_movement: bool,
    pub blood: bool,
    pub headbob: bool,
    pub fov: u8,               // 60-120
    pub vm_fov_scale: bool,
    pub grass_quality: u8,     // 0-3
    pub tree_quality: u8,      // 0-3
    pub rock_quality: u8,      // 0-3
    pub particle_quality: u8,  // 0-3
    pub decor_quality: u8,     // 0-3
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            quality: 2,
            shadow_quality: 1,
            shadow_cascades: false,
            water_quality: 1,
            water_reflections: false,
            lod_bias: 1.0,
            draw_distance: 2000.0,
            anisotropic: 4,
            anti_aliasing: 1,
            motion_blur: false,
            depth_of_field: false,
            bloom: false,
            lens_dirt: false,
            eye_blink: false,
            eye_movement: false,
            blood: false,
            headbob: false,
            fov: 90,
            vm_fov_scale: true,
            grass_quality: 1,
            tree_quality: 1,
            rock_quality: 1,
            particle_quality: 1,
            decor_quality: 1,
        }
    }
}

impl GraphicsConfig {
    /// Generate console commands for in-game config file
    pub fn to_console_commands(&self) -> Vec<String> {
        let aa_str = match self.anti_aliasing {
            0 => "0", 1 => "1", 2 => "2", 3 => "3", _ => "1",
        };
        let aniso_str = match self.anisotropic {
            0 => "0", 1 => "2", 2 => "4", 3 => "8", 4 => "16", _ => "4",
        };

        vec![
            format!("graphics.quality {}", self.quality),
            format!("graphics.shadowquality {}", self.shadow_quality),
            format!("graphics.shadowcascades {}", if self.shadow_cascades { 1 } else { 0 }),
            format!("graphics.waterquality {}", self.water_quality),
            format!("graphics.waterreflection {}", if self.water_reflections { 1 } else { 0 }),
            format!("graphics.lodbias {:.2}", self.lod_bias),
            format!("graphics.drawdistance {:.0}", self.draw_distance),
            format!("graphics.anisotropic {}", aniso_str),
            format!("graphics.antialias {}", aa_str),
            format!("graphics.motionblur {}", if self.motion_blur { 1 } else { 0 }),
            format!("graphics.dof {}", if self.depth_of_field { 1 } else { 0 }),
            format!("graphics.bloom {}", if self.bloom { 1 } else { 0 }),
            format!("graphics.lensdirt {}", if self.lens_dirt { 1 } else { 0 }),
            format!("graphics.eyeblink {}", if self.eye_blink { 1 } else { 0 }),
            format!("graphics.eyemovement {}", if self.eye_movement { 1 } else { 0 }),
            format!("blood {}", if self.blood { 1 } else { 0 }),
            format!("graphics.headbob {}", if self.headbob { 1 } else { 0 }),
            format!("graphics.fov {}", self.fov),
            format!("vm_fov_scale {}", if self.vm_fov_scale { 1 } else { 0 }),
            format!("grass.quality {}", self.grass_quality),
            format!("tree.quality {}", self.tree_quality),
            format!("rock.quality {}", self.rock_quality),
            format!("particle.quality {}", self.particle_quality),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemTweaks {
    // Performance
    pub ultimate_power_plan: bool,
    pub game_mode: bool,
    pub hardware_gpu_scheduling: bool,
    pub disable_xbox_game_bar: bool,
    pub disable_fullscreen_optimizations: bool,
    pub set_high_timer_res: bool,

    // Parasite processes
    pub kill_xbox_services: bool,
    pub kill_superfetch: bool,
    pub kill_windows_search: bool,
    pub kill_print_spooler: bool,
    pub kill_fax: bool,

    // Network
    pub optimize_network: bool,
    pub disable_nagle: bool,

    // Privacy / telemetry
    pub disable_telemetry: bool,
    pub disable_tips: bool,

    pub tweaks_applied: bool,
    pub tweaks_backup: Option<SystemTweaksBackup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTweaksBackup {
    pub timestamp: String,
    pub previous_power_plan: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HardwareInfo {
    pub ram_total_mb: u64,
    pub cpu_cores: u32,
    pub cpu_name: String,
    pub gpu_name: String,
    pub os_version: String,
    pub is_nvidia: bool,
    pub is_amd: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub launch_options: LaunchOptions,
    pub graphics: GraphicsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WindowState {
    pub active_tab: usize,
}

/// Notification shown to user
#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub kind: NotificationKind,
    pub timestamp: std::time::Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationKind {
    Success,
    Warning,
    Error,
    Info,
}
