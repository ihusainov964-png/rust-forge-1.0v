// config/mod.rs — All configuration data structures with serde serialization

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cheap early read of just the theme preference, used by main.rs to pick
/// the matching window icon before the egui context exists.
pub fn load_and_peek_theme() -> bool {
    crate::core::persistence::load_config().window_state.frieren_theme
}

/// Complete app state saved to disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub launch_options: LaunchOptions,
    pub graphics: GraphicsConfig,
    pub advanced: AdvancedTweaks,
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
            advanced: AdvancedTweaks::default(),
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

/// Extra client-side convar toggles for FPS-hungry background effects that
/// have no slider in the normal Rust options menu. Everything here is a
/// client convar (graphics.*, effects.*, audio.*) written to client.cfg —
/// nothing touches server convars, process memory, or EAC-protected values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedTweaks {
    // ── Мир и окружение ──────────────────────────────────────────────
    pub fog_detail: bool,
    pub cloud_detail: bool,
    pub wind_simulation: bool,
    pub dust_particles: bool,
    pub ambient_occlusion: bool,
    pub sun_shafts: bool,

    // ── Экранные пост-эффекты ────────────────────────────────────────
    pub vignette: bool,
    pub chromatic_aberration: bool,
    pub film_grain: bool,
    pub lens_flare: bool,
    pub screen_space_reflections: bool,
    pub color_grading_lut: bool,

    // ── Тени и детализация ───────────────────────────────────────────
    pub contact_shadows: bool,
    pub mesh_shadow_distance: bool,
    pub shader_lod_full: bool,
    pub terrain_detail_recalc: bool,
    pub grass_displacement: bool,

    // ── Частицы и декали ─────────────────────────────────────────────
    pub blood_decals: bool,
    pub bullet_impact_decals: bool,
    pub explosion_particles: bool,
    pub muzzle_flash_particles: bool,
    pub footstep_dust: bool,
    pub fire_smoke_particles: bool,

    // ── Физика объектов и мира ───────────────────────────────────────
    pub ragdoll_physics_detail: bool,
    pub corpse_extended_lifetime: bool,
    pub cloth_physics: bool,
    pub foliage_wind_sway: bool,
    pub deployable_shadow_detail: bool,

    // ── Звук и амбиент ────────────────────────────────────────────────
    pub ambience_layer: bool,
    pub wind_audio: bool,
    pub wildlife_audio: bool,
    pub spatial_voice_processing: bool,
    pub footstep_reverb: bool,

    // ── Рендер и сеть визуалов ────────────────────────────────────────
    pub water_simulation_detail: bool,
    pub super_sampling: bool,
    pub post_process_aa: bool,
    pub prop_pooling_optimization: bool,

    // ── Прочее ──────────────────────────────────────────────────────
    pub camera_shake: bool,
    pub combat_text_popups: bool,
    pub loot_glow_effects: bool,
    pub aggressive_shadow_lod: bool,

    /// Свои convar'ы — по одному на строку, например: graphics.foo 0
    /// Найти новые можно в игре: F1 → find <слово>
    pub custom_convars: String,

    /// Master gate: only `graphics.aggressiveShadowLod` (verified against
    /// official Facepunch patch notes) applies by default. The other 40
    /// toggles are named by convention, not individually confirmed against
    /// the live game — they must be explicitly acknowledged before they're
    /// written to client.cfg, since a wrong guess about a real convar's
    /// value range can hurt performance instead of helping it.
    pub experimental_acknowledged: bool,
}

impl Default for AdvancedTweaks {
    fn default() -> Self {
        // Все включено по умолчанию (как в ванильной игре) — пользователь
        // сам решает, что отключить ради FPS.
        Self {
            fog_detail: true,
            cloud_detail: true,
            wind_simulation: true,
            dust_particles: true,
            ambient_occlusion: true,
            sun_shafts: true,

            vignette: true,
            chromatic_aberration: true,
            film_grain: true,
            lens_flare: true,
            screen_space_reflections: true,
            color_grading_lut: true,

            contact_shadows: true,
            mesh_shadow_distance: true,
            shader_lod_full: true,
            terrain_detail_recalc: true,
            grass_displacement: true,

            blood_decals: true,
            bullet_impact_decals: true,
            explosion_particles: true,
            muzzle_flash_particles: true,
            footstep_dust: true,
            fire_smoke_particles: true,

            ragdoll_physics_detail: true,
            corpse_extended_lifetime: true,
            cloth_physics: true,
            foliage_wind_sway: true,
            deployable_shadow_detail: true,

            ambience_layer: true,
            wind_audio: true,
            wildlife_audio: true,
            spatial_voice_processing: true,
            footstep_reverb: true,

            water_simulation_detail: true,
            super_sampling: true,
            post_process_aa: true,
            prop_pooling_optimization: true,

            camera_shake: true,
            combat_text_popups: true,
            loot_glow_effects: true,
            aggressive_shadow_lod: true,
            custom_convars: String::new(),
            experimental_acknowledged: false,
        }
    }
}

impl AdvancedTweaks {
    /// How many of the 40 toggles are currently OFF (disabled for FPS)
    pub fn disabled_count(&self) -> usize {
        let flags = [
            self.fog_detail, self.cloud_detail, self.wind_simulation, self.dust_particles,
            self.ambient_occlusion, self.sun_shafts,
            self.vignette, self.chromatic_aberration, self.film_grain, self.lens_flare,
            self.screen_space_reflections, self.color_grading_lut,
            self.contact_shadows, self.mesh_shadow_distance, self.shader_lod_full,
            self.terrain_detail_recalc, self.grass_displacement,
            self.blood_decals, self.bullet_impact_decals, self.explosion_particles,
            self.muzzle_flash_particles, self.footstep_dust, self.fire_smoke_particles,
            self.ragdoll_physics_detail, self.corpse_extended_lifetime, self.cloth_physics,
            self.foliage_wind_sway, self.deployable_shadow_detail,
            self.ambience_layer, self.wind_audio, self.wildlife_audio,
            self.spatial_voice_processing, self.footstep_reverb,
            self.water_simulation_detail, self.super_sampling, self.post_process_aa,
            self.prop_pooling_optimization,
            self.camera_shake, self.combat_text_popups, self.loot_glow_effects,
            self.aggressive_shadow_lod,
        ];
        flags.iter().filter(|on| !**on).count()
    }

    /// Total number of toggles this tab exposes
    pub const TOTAL: usize = 41;

    /// Every toggle ON (vanilla game defaults)
    pub fn all_on() -> Self {
        Self::default()
    }

    /// Every toggle OFF (maximum FPS, minimum effects)
    pub fn all_off() -> Self {
        Self {
            fog_detail: false, cloud_detail: false, wind_simulation: false, dust_particles: false,
            ambient_occlusion: false, sun_shafts: false,
            vignette: false, chromatic_aberration: false, film_grain: false, lens_flare: false,
            screen_space_reflections: false, color_grading_lut: false,
            contact_shadows: false, mesh_shadow_distance: false, shader_lod_full: false,
            terrain_detail_recalc: false, grass_displacement: false,
            blood_decals: false, bullet_impact_decals: false, explosion_particles: false,
            muzzle_flash_particles: false, footstep_dust: false, fire_smoke_particles: false,
            ragdoll_physics_detail: false, corpse_extended_lifetime: false, cloth_physics: false,
            foliage_wind_sway: false, deployable_shadow_detail: false,
            ambience_layer: false, wind_audio: false, wildlife_audio: false,
            spatial_voice_processing: false, footstep_reverb: false,
            water_simulation_detail: false, super_sampling: false, post_process_aa: false,
            prop_pooling_optimization: false,
            camera_shake: false, combat_text_popups: false, loot_glow_effects: false,
            aggressive_shadow_lod: false,
            custom_convars: String::new(),
            experimental_acknowledged: false,
        }
    }

    fn b(v: bool) -> &'static str { if v { "1" } else { "0" } }

    /// Generate client.cfg convar lines. These are community-documented
    /// client convars (graphics.*, effects.*, audio.*) — always run only on
    /// your own machine, never touch server.* or sv_cheats-gated values, so
    /// they can't trigger EAC. Game updates can rename/remove a convar; use
    /// `find <keyword>` in the in-game F1 console to double-check if one
    /// stops having an effect.
    pub fn to_console_commands(&self) -> Vec<String> {
        // Only this one is confirmed against official Facepunch patch notes.
        // It always applies — it's safe.
        let mut cmds = vec![
            format!("graphics.aggressiveShadowLod {}", Self::b(self.aggressive_shadow_lod)),
        ];

        // The other 40 are named by convention, not individually verified —
        // after the July 2026 FPS regression, they only get written once the
        // user has ticked "I understand these aren't fully verified" below.
        if self.experimental_acknowledged {
            cmds.extend(vec![
            format!("graphics.fog {}", Self::b(self.fog_detail)),
            format!("graphics.clouds {}", Self::b(self.cloud_detail)),
            format!("wind.enabled {}", Self::b(self.wind_simulation)),
            format!("effects.dust {}", Self::b(self.dust_particles)),
            format!("graphics.ao {}", Self::b(self.ambient_occlusion)),
            format!("graphics.sunshafts {}", Self::b(self.sun_shafts)),

            format!("graphics.vignette {}", Self::b(self.vignette)),
            format!("graphics.chromaticaberration {}", Self::b(self.chromatic_aberration)),
            format!("graphics.filmgrain {}", Self::b(self.film_grain)),
            format!("graphics.lensflare {}", Self::b(self.lens_flare)),
            format!("graphics.ssr {}", Self::b(self.screen_space_reflections)),
            format!("graphics.colorgrading {}", Self::b(self.color_grading_lut)),

            format!("graphics.contactshadows {}", Self::b(self.contact_shadows)),
            format!("mesh.shadowdistance {}", if self.mesh_shadow_distance { "150" } else { "0" }),
            format!("graphics.shaderlod {}", if self.shader_lod_full { "2000" } else { "600" }),
            format!("terrain.idleinterval {}", if self.terrain_detail_recalc { "1" } else { "4" }),
            format!("grass.displacement {}", Self::b(self.grass_displacement)),

            format!("effects.blooddecals {}", Self::b(self.blood_decals)),
            format!("effects.bulletdecals {}", Self::b(self.bullet_impact_decals)),
            format!("effects.explosionparticles {}", Self::b(self.explosion_particles)),
            format!("effects.muzzleflash {}", Self::b(self.muzzle_flash_particles)),
            format!("effects.footstepdust {}", Self::b(self.footstep_dust)),
            format!("effects.firesmoke {}", Self::b(self.fire_smoke_particles)),

            format!("ragdoll.detail {}", Self::b(self.ragdoll_physics_detail)),
            format!("corpse.extendedlifetime {}", Self::b(self.corpse_extended_lifetime)),
            format!("graphics.clothphysics {}", Self::b(self.cloth_physics)),
            format!("graphics.foliagesway {}", Self::b(self.foliage_wind_sway)),
            format!("graphics.deployableshadows {}", Self::b(self.deployable_shadow_detail)),

            format!("ambience.enabled {}", Self::b(self.ambience_layer)),
            format!("audio.wind {}", Self::b(self.wind_audio)),
            format!("audio.wildlife {}", Self::b(self.wildlife_audio)),
            format!("audio.spatialvoice {}", Self::b(self.spatial_voice_processing)),
            format!("audio.footstepreverb {}", Self::b(self.footstep_reverb)),

            format!("water.simdetail {}", Self::b(self.water_simulation_detail)),
            format!("gfx.ssaa {}", Self::b(self.super_sampling)),
            format!("effects.aa_quality {}", Self::b(self.post_process_aa)),
            format!("graphics.proppooling {}", Self::b(self.prop_pooling_optimization)),

            format!("graphics.camerashake {}", Self::b(self.camera_shake)),
            format!("ui.combattext {}", Self::b(self.combat_text_popups)),
            format!("effects.lootglow {}", Self::b(self.loot_glow_effects)),
            ]);
        }

        // Custom convars always apply — the user typed these themselves
        // after finding them in the live game console, so they're trusted.
        for line in self.custom_convars.lines() {
            let line = line.trim();
            if !line.is_empty() {
                cmds.push(line.to_string());
            }
        }

        cmds
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
    pub kill_remote_registry: bool,
    pub kill_tablet_input: bool,
    pub kill_secondary_logon: bool,
    pub kill_diagnostic_policy: bool,
    pub kill_downloaded_maps: bool,

    // Network
    pub optimize_network: bool,
    pub disable_nagle: bool,
    pub disable_qos: bool,
    pub optimize_dns: bool,
    pub disable_network_throttling: bool,

    // Privacy / telemetry
    pub disable_telemetry: bool,
    pub disable_tips: bool,
    pub disable_activity_history: bool,
    pub disable_location: bool,
    pub disable_cortana: bool,
    pub disable_compat_telemetry: bool,

    // Visual effects
    pub disable_animations: bool,
    pub disable_transparency: bool,
    pub classic_menu: bool,

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
    /// Alternate soft lavender/sage colour theme, inspired by the calm
    /// travel-journal mood of "Frieren: Beyond Journey's End" — an
    /// original palette, no copyrighted artwork or characters involved.
    pub frieren_theme: bool,
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
