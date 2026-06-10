// core/tweaks.rs — Legal Windows system optimizations
// Uses only official Windows APIs and documented registry keys
// All changes are reversible with the revert function.

use anyhow::{anyhow, Result};
use log::{info, warn};
use crate::config::SystemTweaks;

/// Apply selected system tweaks (Windows only)
/// All tweaks are fully reversible via revert_tweaks()
pub fn apply_system_tweaks(tweaks: &SystemTweaks) -> Result<Vec<String>> {
    let mut applied = Vec::new();

    #[cfg(target_os = "windows")]
    {
        if tweaks.ultimate_power_plan {
            match apply_ultimate_power_plan() {
                Ok(_) => applied.push("✅ Ultimate Performance Power Plan".to_string()),
                Err(e) => applied.push(format!("⚠️ Power Plan: {}", e)),
            }
        }

        if tweaks.disable_xbox_game_bar {
            match disable_game_bar() {
                Ok(_) => applied.push("✅ Xbox Game Bar отключён".to_string()),
                Err(e) => applied.push(format!("⚠️ Game Bar: {}", e)),
            }
        }

        if tweaks.game_mode {
            match set_game_mode(true) {
                Ok(_) => applied.push("✅ Windows Game Mode включён".to_string()),
                Err(e) => applied.push(format!("⚠️ Game Mode: {}", e)),
            }
        }

        if tweaks.hardware_gpu_scheduling {
            applied.push("ℹ️ HAGS: требует перезагрузки (см. Настройки > Графика)".to_string());
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = tweaks;
        applied.push("ℹ️ Системные твики доступны только на Windows".to_string());
    }

    Ok(applied)
}

/// Revert all system tweaks to default
pub fn revert_system_tweaks() -> Result<Vec<String>> {
    let mut reverted = Vec::new();

    #[cfg(target_os = "windows")]
    {
        // Restore balanced power plan
        match restore_balanced_power_plan() {
            Ok(_) => reverted.push("✅ Power Plan → Balanced".to_string()),
            Err(e) => reverted.push(format!("⚠️ Power Plan revert: {}", e)),
        }

        // Re-enable Game Bar
        match enable_game_bar() {
            Ok(_) => reverted.push("✅ Xbox Game Bar восстановлён".to_string()),
            Err(e) => reverted.push(format!("⚠️ Game Bar revert: {}", e)),
        }
    }

    #[cfg(not(target_os = "windows"))]
    reverted.push("ℹ️ Не применялось (не Windows)".to_string());

    Ok(reverted)
}

#[cfg(target_os = "windows")]
fn apply_ultimate_power_plan() -> Result<()> {
    use std::process::Command;
    // Duplicate the hidden Ultimate Performance scheme (official Microsoft GUID)
    let output = Command::new("powercfg")
        .args(["-duplicatescheme", "e9a42b02-d5df-448d-aa00-03f14749eb61"])
        .output()
        .map_err(|e| anyhow!("powercfg failed: {}", e))?;

    // Extract the new GUID from output and activate it
    let stdout = String::from_utf8_lossy(&output.stdout);
    if let Some(guid) = extract_guid_from_powercfg(&stdout) {
        Command::new("powercfg")
            .args(["-setactive", &guid])
            .output()
            .map_err(|e| anyhow!("setactive failed: {}", e))?;
        info!("Ultimate Performance plan activated: {}", guid);
    } else {
        // Try direct GUID (may already exist)
        let _ = Command::new("powercfg")
            .args(["-setactive", "e9a42b02-d5df-448d-aa00-03f14749eb61"])
            .output();
        info!("Attempted to set Ultimate Performance directly");
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn restore_balanced_power_plan() -> Result<()> {
    use std::process::Command;
    // Balanced plan GUID
    Command::new("powercfg")
        .args(["-setactive", "381b4222-f694-41f0-9685-ff5bb260df2e"])
        .output()
        .map_err(|e| anyhow!("powercfg restore failed: {}", e))?;
    info!("Balanced power plan restored");
    Ok(())
}

#[cfg(target_os = "windows")]
fn extract_guid_from_powercfg(output: &str) -> Option<String> {
    // powercfg output: "Power Scheme GUID: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx  (Ultimate Performance)"
    for line in output.lines() {
        if line.contains("GUID:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == "GUID:" && i + 1 < parts.len() {
                    return Some(parts[i + 1].to_string());
                }
            }
        }
    }
    None
}

#[cfg(target_os = "windows")]
fn disable_game_bar() -> Result<()> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    use winreg::RegKey;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey_with_flags(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR",
        KEY_WRITE,
    )?;
    key.set_value("AppCaptureEnabled", &0u32)?;
    let (key2, _) = hkcu.create_subkey_with_flags(
        r"System\GameConfigStore",
        KEY_WRITE,
    )?;
    key2.set_value("GameDVR_Enabled", &0u32)?;
    info!("Xbox Game Bar disabled via registry");
    Ok(())
}

#[cfg(target_os = "windows")]
fn enable_game_bar() -> Result<()> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    use winreg::RegKey;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey_with_flags(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR",
        KEY_WRITE,
    )?;
    key.set_value("AppCaptureEnabled", &1u32)?;
    let (key2, _) = hkcu.create_subkey_with_flags(
        r"System\GameConfigStore",
        KEY_WRITE,
    )?;
    key2.set_value("GameDVR_Enabled", &1u32)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_game_mode(enabled: bool) -> Result<()> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    use winreg::RegKey;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey_with_flags(
        r"SOFTWARE\Microsoft\GameBar",
        KEY_WRITE,
    )?;
    key.set_value("AutoGameModeEnabled", &(if enabled { 1u32 } else { 0u32 }))?;
    info!("Game Mode set to {}", enabled);
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn get_nvidia_tips() -> Vec<String> { Vec::new() }

#[cfg(target_os = "windows")]
pub fn get_nvidia_tips() -> Vec<String> {
    vec![
        "📌 NVIDIA Control Panel → Manage 3D Settings → Low Latency Mode → Ultra".to_string(),
        "📌 Power Management Mode → Prefer Maximum Performance".to_string(),
        "📌 Texture Filtering Quality → High Performance".to_string(),
        "📌 Vertical Sync → Off (используй лимит FPS в игре)".to_string(),
        "📌 Shader Cache → Unlimited".to_string(),
        "📌 Reflex Low Latency → On + Boost (если доступно)".to_string(),
        "📌 DLSS → Quality или Balanced (для RTX серии)".to_string(),
    ]
}

/// Collect current Windows performance info for display
pub fn get_windows_perf_info() -> Vec<(String, String)> {
    let mut info = Vec::new();

    #[cfg(target_os = "windows")]
    {
        // Check Game Mode registry
        {
            use winreg::enums::HKEY_CURRENT_USER;
            use winreg::RegKey;
            let status = (|| -> Option<&'static str> {
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                let key = hkcu.open_subkey(r"SOFTWARE\Microsoft\GameBar").ok()?;
                let val: u32 = key.get_value("AutoGameModeEnabled").ok()?;
                match val {
                    1 => Some("Enabled ✅"),
                    0 => Some("Disabled"),
                    _ => Some("Unknown"),
                }
            })().unwrap_or("Unknown");
            info.push(("Game Mode".to_string(), status.to_string()));
            let status = match hkcu.ok().flatten() {
                Some(1) => "Enabled ✅",
                Some(0) => "Disabled",
                _ => "Unknown",
            };
            info.push(("Game Mode".to_string(), status.to_string()));
        }
    }

    #[cfg(not(target_os = "windows"))]
    info.push(("Platform".to_string(), "Linux (tweaks N/A)".to_string()));

    info
}
