// core/steam.rs — Steam detection and game launch

use anyhow::{anyhow, Result};
use log::{info, warn};
use std::path::PathBuf;

const RUST_APP_ID: &str = "252490";

/// Find Steam installation path
pub fn find_steam_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        // Try registry first
        if let Ok(path) = read_steam_path_from_registry() {
            let p = PathBuf::from(path);
            if p.exists() {
                return Some(p);
            }
        }
        // Common fallback paths
        let candidates = [
            r"C:\Program Files (x86)\Steam",
            r"C:\Program Files\Steam",
            r"D:\Steam",
            r"D:\SteamLibrary",
        ];
        for c in &candidates {
            let p = PathBuf::from(c);
            if p.exists() {
                return Some(p);
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        let home = dirs::home_dir()?;
        let candidates = [
            home.join(".steam/steam"),
            home.join(".local/share/Steam"),
        ];
        for c in &candidates {
            if c.exists() {
                return Some(c.clone());
            }
        }
    }
    None
}

#[cfg(target_os = "windows")]
fn read_steam_path_from_registry() -> Result<String> {
    use winreg::enums::HKEY_LOCAL_MACHINE;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let steam_key = hklm.open_subkey(r"SOFTWARE\WOW6432Node\Valve\Steam")?;
    let path: String = steam_key.get_value("InstallPath")?;
    Ok(path)
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
fn read_steam_path_from_registry() -> Result<String> {
    Err(anyhow!("Registry not available on this platform"))
}

/// Check if Steam is running
pub fn is_steam_running() -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("tasklist").output() {
            let s = String::from_utf8_lossy(&output.stdout);
            return s.to_lowercase().contains("steam.exe");
        }
    }
    false
}

/// Launch Rust via Steam URL protocol with given launch options
pub fn launch_rust_via_steam(launch_options_str: &str) -> Result<()> {
    info!("Launching Rust with options: {}", launch_options_str);

    // Write launch options to Steam config before launching
    apply_steam_launch_options(launch_options_str)?;

    // Use steam:// protocol URL to launch
    let steam_url = format!("steam://run/{}", RUST_APP_ID);

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("cmd")
            .args(["/c", "start", "", &steam_url])
            .spawn()
            .map_err(|e| anyhow!("Failed to launch Steam URL: {}", e))?;
        info!("Rust launched via Steam URL");
        return Ok(());
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("xdg-open")
            .arg(&steam_url)
            .spawn()
            .map_err(|e| anyhow!("Failed to launch via xdg-open: {}", e))?;
        return Ok(());
    }

    #[allow(unreachable_code)]
    Err(anyhow!("Unsupported platform"))
}

/// Write launch options to Steam's localconfig.vdf
/// This is the standard way to set launch options — same as Steam UI does it.
pub fn apply_steam_launch_options(options: &str) -> Result<()> {
    let steam_path = find_steam_path()
        .ok_or_else(|| anyhow!("Steam installation not found"))?;

    let userdata_path = steam_path.join("userdata");
    if !userdata_path.exists() {
        warn!("Steam userdata directory not found, skipping config write");
        return Ok(());
    }

    // Find all user directories and update localconfig.vdf
    let entries = std::fs::read_dir(&userdata_path)
        .map_err(|e| anyhow!("Cannot read userdata: {}", e))?;

    let mut updated = false;
    for entry in entries.flatten() {
        let local_config = entry.path().join("config").join("localconfig.vdf");
        if local_config.exists() {
            match update_local_config(&local_config, options) {
                Ok(_) => {
                    info!("Updated launch options in {:?}", local_config);
                    updated = true;
                }
                Err(e) => {
                    warn!("Failed to update {:?}: {}", local_config, e);
                }
            }
        }
    }

    if !updated {
        info!("No localconfig.vdf found; launch options will use custom args approach");
    }

    Ok(())
}

/// Backup and update localconfig.vdf with new launch options for Rust
fn update_local_config(path: &std::path::Path, new_options: &str) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| anyhow!("Cannot read localconfig.vdf: {}", e))?;

    // Create backup
    let backup_path = path.with_extension("vdf.rfbackup");
    if !backup_path.exists() {
        std::fs::write(&backup_path, &content)
            .map_err(|e| anyhow!("Cannot write backup: {}", e))?;
        info!("Backup created at {:?}", backup_path);
    }

    // Simple VDF key replacement for LaunchOptions under app 252490
    // This is the same format Steam itself uses
    let updated = inject_launch_options_vdf(&content, RUST_APP_ID, new_options);

    std::fs::write(path, updated)
        .map_err(|e| anyhow!("Cannot write localconfig.vdf: {}", e))?;

    Ok(())
}

/// Inject or replace LaunchOptions value in Steam VDF format for given app ID
fn inject_launch_options_vdf(vdf: &str, app_id: &str, options: &str) -> String {
    let app_section = format!("\"{}\"", app_id);
    let launch_key = "\"LaunchOptions\"";
    let new_entry = format!("\t\t\t\t\t\"LaunchOptions\"\t\t\"{}\"", options);

    let mut result = String::with_capacity(vdf.len() + 128);
    let mut in_app_section = false;
    let mut found_app = false;
    let mut replaced = false;
    let mut depth = 0i32;
    let mut app_depth = -1i32;

    for line in vdf.lines() {
        let trimmed = line.trim();

        if !found_app && trimmed == app_section {
            found_app = true;
            in_app_section = true;
            app_depth = depth;
            result.push_str(line);
            result.push('\n');
            continue;
        }

        if trimmed == "{" { depth += 1; }
        if trimmed == "}" {
            if in_app_section && depth == app_depth + 1 && !replaced {
                // Closing brace of app section — inject LaunchOptions before it
                result.push_str(&new_entry);
                result.push('\n');
                replaced = true;
            }
            depth -= 1;
            if depth <= app_depth {
                in_app_section = false;
            }
        }

        if in_app_section && trimmed.starts_with(launch_key) {
            // Replace existing entry
            result.push_str(&new_entry);
            result.push('\n');
            replaced = true;
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    result
}

/// Restore original launch options from backup
pub fn restore_launch_options_backup() -> Result<()> {
    let steam_path = find_steam_path()
        .ok_or_else(|| anyhow!("Steam not found"))?;

    let userdata_path = steam_path.join("userdata");
    let entries = std::fs::read_dir(&userdata_path)
        .map_err(|e| anyhow!("Cannot read userdata: {}", e))?;

    for entry in entries.flatten() {
        let local_config = entry.path().join("config").join("localconfig.vdf");
        let backup = local_config.with_extension("vdf.rfbackup");
        if backup.exists() {
            std::fs::copy(&backup, &local_config)
                .map_err(|e| anyhow!("Cannot restore backup: {}", e))?;
            info!("Restored backup from {:?}", backup);
        }
    }
    Ok(())
}

/// Get Rust config directory path
pub fn get_rust_config_path() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        // %APPDATA%\..\LocalLow\Facepunch Studios\Rust\cfg
        if let Some(appdata) = dirs::data_dir() {
            let path = appdata
                .parent()?
                .join("LocalLow")
                .join("Facepunch Studios")
                .join("Rust")
                .join("cfg");
            if path.exists() {
                return Some(path);
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Some(home) = dirs::home_dir() {
            let path = home
                .join(".config/unity3d/Facepunch Studios/Rust/cfg");
            if path.exists() {
                return Some(path);
            }
        }
    }
    None
}

/// Write graphics console commands to Rust's client.cfg
pub fn write_rust_config(commands: &[String]) -> Result<PathBuf> {
    let cfg_dir = get_rust_config_path()
        .or_else(|| {
            // If cfg dir doesn't exist, create it
            #[cfg(target_os = "windows")]
            {
                dirs::data_dir().and_then(|d| {
                    let p = d.parent()?.join("LocalLow").join("Facepunch Studios").join("Rust").join("cfg");
                    std::fs::create_dir_all(&p).ok()?;
                    Some(p)
                })
            }
            #[cfg(not(target_os = "windows"))]
            { None }
        })
        .ok_or_else(|| anyhow!("Cannot find or create Rust config directory. Make sure Rust has been launched at least once."))?;

    let config_file = cfg_dir.join("client.cfg");

    // Backup existing config
    if config_file.exists() {
        let backup = cfg_dir.join("client.cfg.rfbackup");
        if !backup.exists() {
            std::fs::copy(&config_file, &backup)?;
            info!("Backed up client.cfg");
        }
    }

    let content = format!(
        "// RustForge Generated Config — {}\n// Do not edit manually while RustForge is running\n\n{}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M"),
        commands.join("\n")
    );

    std::fs::write(&config_file, &content)
        .map_err(|e| anyhow!("Cannot write client.cfg: {}", e))?;

    info!("Written {} commands to {:?}", commands.len(), config_file);
    Ok(config_file)
}

/// Clear Rust shader cache and asset bundles (legal performance reset)
pub fn clear_rust_cache() -> Result<usize> {
    let mut cleared = 0usize;

    #[cfg(target_os = "windows")]
    {
        // Rust shader cache locations
        let cache_paths = [
            std::env::var("LOCALAPPDATA").ok()
                .map(|a| PathBuf::from(a).join("Temp").join("Unity")),
            dirs::data_dir().and_then(|d| {
                Some(d.parent()?.join("LocalLow").join("Facepunch Studios").join("Rust").join("shaders"))
            }),
        ];

        for maybe_path in cache_paths.iter().flatten() {
            if maybe_path.exists() {
                if let Ok(entries) = std::fs::read_dir(maybe_path) {
                    for entry in entries.flatten() {
                        if std::fs::remove_file(entry.path()).is_ok() {
                            cleared += 1;
                        }
                    }
                }
            }
        }
    }

    info!("Cleared {} cache files", cleared);
    Ok(cleared)
}
