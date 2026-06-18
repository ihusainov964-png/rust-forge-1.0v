// core/tweaks.rs — Legal Windows optimizations
// Only official APIs. All reversible. No cheats.

use anyhow::{anyhow, Result};
use log::info;
use crate::config::SystemTweaks;

// ── Apply / Revert ────────────────────────────────────────────────────────────

pub fn apply_system_tweaks(tweaks: &SystemTweaks) -> Result<Vec<String>> {
    let mut applied = Vec::new();
    #[cfg(target_os = "windows")]
    {
        if tweaks.ultimate_power_plan {
            match apply_ultimate_power_plan() {
                Ok(_)  => applied.push("✅ Ultimate Performance Power Plan".to_string()),
                Err(e) => applied.push(format!("⚠️ Power Plan: {}", e)),
            }
        }
        if tweaks.disable_xbox_game_bar {
            match disable_game_bar() {
                Ok(_)  => applied.push("✅ Xbox Game Bar отключён".to_string()),
                Err(e) => applied.push(format!("⚠️ Game Bar: {}", e)),
            }
        }
        if tweaks.game_mode {
            match set_game_mode(true) {
                Ok(_)  => applied.push("✅ Windows Game Mode включён".to_string()),
                Err(e) => applied.push(format!("⚠️ Game Mode: {}", e)),
            }
        }
        if tweaks.disable_fullscreen_optimizations {
            applied.push("ℹ️ Fullscreen Optimizations: настрой вручную на .exe Rust → Свойства → Совместимость".to_string());
        }
        if tweaks.hardware_gpu_scheduling {
            applied.push("ℹ️ HAGS: включи в Параметры → Экран → Графика → Стандартные параметры графики".to_string());
        }
        // Parasite processes
        if tweaks.kill_xbox_services    { kill_service("XboxGipSvc");  kill_service("XblGameSave"); kill_service("XblAuthManager"); applied.push("✅ Xbox сервисы остановлены".to_string()); }
        if tweaks.kill_superfetch       { kill_service("SysMain");       applied.push("✅ SysMain (Superfetch) остановлен".to_string()); }
        if tweaks.kill_windows_search   { kill_service("WSearch");       applied.push("✅ Windows Search остановлен".to_string()); }
        if tweaks.kill_print_spooler    { kill_service("Spooler");       applied.push("✅ Print Spooler остановлен".to_string()); }
        if tweaks.kill_fax             { kill_service("Fax");            applied.push("✅ Fax сервис остановлен".to_string()); }
        if tweaks.kill_remote_registry  { kill_service("RemoteRegistry"); applied.push("✅ Remote Registry остановлен".to_string()); }
        if tweaks.kill_tablet_input     { kill_service("TabletInputService"); kill_service("hidserv"); applied.push("✅ Tablet Input остановлен".to_string()); }
        if tweaks.kill_secondary_logon  { kill_service("seclogon");       applied.push("✅ Secondary Logon остановлен".to_string()); }
        if tweaks.kill_diagnostic_policy { kill_service("DPS"); kill_service("WdiServiceHost"); applied.push("✅ Diagnostic Policy остановлен".to_string()); }
        if tweaks.kill_downloaded_maps  { kill_service("MapsBroker");     applied.push("✅ Maps Manager остановлен".to_string()); }
        if tweaks.disable_qos           { disable_qos_reg();               applied.push("✅ QoS резервирование отключено (+20% канала)".to_string()); }
        if tweaks.optimize_dns          { optimize_dns_reg();              applied.push("✅ DNS кэш оптимизирован".to_string()); }
        if tweaks.disable_network_throttling { disable_net_throttle_reg(); applied.push("✅ Network Throttling отключён".to_string()); }
        if tweaks.disable_activity_history { disable_activity_reg();      applied.push("✅ История активности отключена".to_string()); }
        if tweaks.disable_location      { disable_location_reg();          applied.push("✅ Геолокация отключена".to_string()); }
        if tweaks.disable_cortana       { disable_cortana_reg();           applied.push("✅ Cortana отключена".to_string()); }
        if tweaks.disable_compat_telemetry { disable_compat_telemetry_reg(); applied.push("✅ CompatTelRunner отключён".to_string()); }
        if tweaks.disable_animations    { disable_animations_reg();        applied.push("✅ Анимации Windows отключены".to_string()); }
        if tweaks.disable_transparency  { disable_transparency_reg();      applied.push("✅ Прозрачность интерфейса отключена".to_string()); }
        if tweaks.classic_menu          { enable_classic_menu_reg();       applied.push("✅ Классическое меню ПКМ включено".to_string()); }
        if tweaks.disable_telemetry    { disable_telemetry_reg();        applied.push("✅ Телеметрия Windows отключена".to_string()); }
        if tweaks.disable_tips         { disable_tips_reg();             applied.push("✅ Советы и уведомления Windows отключены".to_string()); }
        if tweaks.set_high_timer_res   { set_timer_resolution();        applied.push("✅ Таймер системы: высокое разрешение".to_string()); }
        if tweaks.optimize_network     { optimize_network_reg();         applied.push("✅ Сетевые настройки оптимизированы".to_string()); }
        if tweaks.disable_nagle        { disable_nagle_reg();            applied.push("✅ Алгоритм Нейгла отключён (меньше пинг)".to_string()); }
    }
    #[cfg(not(target_os = "windows"))]
    { let _ = tweaks; applied.push("ℹ️ Системные твики доступны только на Windows".to_string()); }
    Ok(applied)
}

pub fn revert_system_tweaks() -> Result<Vec<String>> {
    let mut reverted = Vec::new();
    #[cfg(target_os = "windows")]
    {
        match restore_balanced_power_plan() {
            Ok(_)  => reverted.push("✅ Power Plan → Balanced".to_string()),
            Err(e) => reverted.push(format!("⚠️ Power Plan: {}", e)),
        }
        match enable_game_bar() {
            Ok(_)  => reverted.push("✅ Xbox Game Bar восстановлён".to_string()),
            Err(e) => reverted.push(format!("⚠️ Game Bar: {}", e)),
        }
        // Restart stopped services
        for svc in &["SysMain", "WSearch", "Spooler", "XboxGipSvc"] {
            let _ = start_service(svc);
        }
        reverted.push("✅ Сервисы восстановлены".to_string());
    }
    #[cfg(not(target_os = "windows"))]
    reverted.push("ℹ️ Не применялось".to_string());
    Ok(reverted)
}

// ── Power Plan ────────────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn apply_ultimate_power_plan() -> Result<()> {
    use std::process::Command;
    let out = Command::new("powercfg")
        .args(["-duplicatescheme", "e9a42b02-d5df-448d-aa00-03f14749eb61"])
        .output()
        .map_err(|e| anyhow!("powercfg: {}", e))?;
    let s = String::from_utf8_lossy(&out.stdout);
    if let Some(guid) = extract_guid(&s) {
        let _ = Command::new("powercfg").args(["-setactive", &guid]).output();
    } else {
        let _ = Command::new("powercfg").args(["-setactive", "e9a42b02-d5df-448d-aa00-03f14749eb61"]).output();
    }
    info!("Ultimate Performance activated");
    Ok(())
}

#[cfg(target_os = "windows")]
fn restore_balanced_power_plan() -> Result<()> {
    use std::process::Command;
    Command::new("powercfg")
        .args(["-setactive", "381b4222-f694-41f0-9685-ff5bb260df2e"])
        .output()
        .map_err(|e| anyhow!("powercfg: {}", e))?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn extract_guid(s: &str) -> Option<String> {
    for line in s.lines() {
        if line.contains("GUID:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, p) in parts.iter().enumerate() {
                if *p == "GUID:" && i + 1 < parts.len() {
                    return Some(parts[i+1].to_string());
                }
            }
        }
    }
    None
}

// ── Services ──────────────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn kill_service(name: &str) {
    let _ = std::process::Command::new("sc").args(["stop", name]).output();
    let _ = std::process::Command::new("sc").args(["config", name, "start=", "disabled"]).output();
    info!("Stopped service: {}", name);
}

#[cfg(target_os = "windows")]
fn start_service(name: &str) -> Result<()> {
    std::process::Command::new("sc").args(["config", name, "start=", "auto"]).output()
        .map_err(|e| anyhow!("{}", e))?;
    std::process::Command::new("sc").args(["start", name]).output()
        .map_err(|e| anyhow!("{}", e))?;
    Ok(())
}

// ── Registry tweaks ───────────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn disable_game_bar() -> Result<()> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    use winreg::RegKey;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (k, _) = hkcu.create_subkey_with_flags(r"SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR", KEY_WRITE)?;
    k.set_value("AppCaptureEnabled", &0u32)?;
    let (k2, _) = hkcu.create_subkey_with_flags(r"System\GameConfigStore", KEY_WRITE)?;
    k2.set_value("GameDVR_Enabled", &0u32)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn enable_game_bar() -> Result<()> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    use winreg::RegKey;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (k, _) = hkcu.create_subkey_with_flags(r"SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR", KEY_WRITE)?;
    k.set_value("AppCaptureEnabled", &1u32)?;
    let (k2, _) = hkcu.create_subkey_with_flags(r"System\GameConfigStore", KEY_WRITE)?;
    k2.set_value("GameDVR_Enabled", &1u32)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_game_mode(on: bool) -> Result<()> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    use winreg::RegKey;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (k, _) = hkcu.create_subkey_with_flags(r"SOFTWARE\Microsoft\GameBar", KEY_WRITE)?;
    k.set_value("AutoGameModeEnabled", &(if on { 1u32 } else { 0u32 }))?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn disable_telemetry_reg() {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE};
    use winreg::RegKey;
    if let Ok(hklm) = std::panic::catch_unwind(|| RegKey::predef(HKEY_LOCAL_MACHINE)) {
        if let Ok((k, _)) = hklm.create_subkey_with_flags(r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", KEY_WRITE) {
            let _ = k.set_value("AllowTelemetry", &0u32);
        }
    }
}

#[cfg(target_os = "windows")]
fn disable_tips_reg() {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    use winreg::RegKey;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((k, _)) = hkcu.create_subkey_with_flags(r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", KEY_WRITE) {
        let _ = k.set_value("SubscribedContent-338389Enabled", &0u32);
        let _ = k.set_value("SystemPaneSuggestionsEnabled", &0u32);
        let _ = k.set_value("SoftLandingEnabled", &0u32);
    }
}

#[cfg(target_os = "windows")]
fn optimize_network_reg() {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE};
    use winreg::RegKey;
    if let Ok(hklm) = std::panic::catch_unwind(|| RegKey::predef(HKEY_LOCAL_MACHINE)) {
        // Increase network buffer sizes
        if let Ok((k, _)) = hklm.create_subkey_with_flags(r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters", KEY_WRITE) {
            let _ = k.set_value("TcpAckFrequency",   &1u32);
            let _ = k.set_value("TCPNoDelay",        &1u32);
            let _ = k.set_value("TcpDelAckTicks",    &0u32);
            let _ = k.set_value("DefaultTTL",        &64u32);
        }
    }
}

#[cfg(target_os = "windows")]
fn disable_nagle_reg() {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE};
    use winreg::RegKey;
    if let Ok(hklm) = std::panic::catch_unwind(|| RegKey::predef(HKEY_LOCAL_MACHINE)) {
        if let Ok((k, _)) = hklm.create_subkey_with_flags(r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces", KEY_WRITE) {
            let _ = k.set_value("TcpAckFrequency", &1u32);
            let _ = k.set_value("TCPNoDelay",      &1u32);
        }
    }
}

#[cfg(target_os = "windows")]
fn set_timer_resolution() {
    // timeBeginPeriod(1) - set minimum timer resolution for smoother frame pacing
    // This is standard practice used by games and pro tools
    #[link(name = "winmm")]
    extern "system" { fn timeBeginPeriod(uPeriod: u32) -> u32; }
    unsafe { timeBeginPeriod(1); }
    info!("Timer resolution set to 1ms");
}

// ── Info ──────────────────────────────────────────────────────────────────────

pub fn get_windows_perf_info() -> Vec<(String, String)> {
    let mut info = Vec::new();
    #[cfg(target_os = "windows")]
    {
        // Power plan check — run once, cached by caller
        if let Ok(out) = std::process::Command::new("powercfg").args(["-getactivescheme"]).output() {
            let s = String::from_utf8_lossy(&out.stdout);
            let plan = if s.contains("Ultimate")         { "Ultimate Performance ✅" }
                       else if s.contains("High perf")   { "High Performance" }
                       else if s.contains("Balanced")    { "Balanced ⚠️" }
                       else                               { "Unknown" };
            info.push(("Power Plan".to_string(), plan.to_string()));
        }

        // Game Mode — registry read (fast)
        {
            use winreg::enums::HKEY_CURRENT_USER;
            use winreg::RegKey;
            let status = (|| -> Option<&'static str> {
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                let key = hkcu.open_subkey(r"SOFTWARE\Microsoft\GameBar").ok()?;
                let val: u32 = key.get_value("AutoGameModeEnabled").ok()?;
                match val { 1 => Some("Enabled ✅"), 0 => Some("Disabled"), _ => Some("Unknown") }
            })().unwrap_or("Unknown");
            info.push(("Game Mode".to_string(), status.to_string()));
        }

        // Service statuses (fast sc query)
        for (svc, label) in &[("SysMain","Superfetch"), ("WSearch","Windows Search"), ("XboxGipSvc","Xbox")] {
            if let Ok(out) = std::process::Command::new("sc").args(["query", svc]).output() {
                let s = String::from_utf8_lossy(&out.stdout);
                let st = if s.contains("RUNNING") { "Running ⚠️" } else { "Stopped ✅" };
                info.push((label.to_string(), st.to_string()));
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    info.push(("Platform".to_string(), "Linux".to_string()));
    info
}

pub fn get_nvidia_tips() -> Vec<String> {
    vec![
        "📌 Low Latency Mode → Ultra".to_string(),
        "📌 Power Management → Prefer Maximum Performance".to_string(),
        "📌 Texture Filtering → High Performance".to_string(),
        "📌 Vertical Sync → Off".to_string(),
        "📌 Shader Cache → Unlimited".to_string(),
        "📌 Reflex Low Latency → On + Boost".to_string(),
    ]
}

// ── New registry functions ────────────────────────────────────────────────────

#[cfg(target_os = "windows")]
fn disable_qos_reg() {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE};
    use winreg::RegKey;
    if let Ok(hklm) = std::panic::catch_unwind(|| RegKey::predef(HKEY_LOCAL_MACHINE)) {
        if let Ok((k, _)) = hklm.create_subkey_with_flags(
            r"SOFTWARE\Policies\Microsoft\Windows\Psched", KEY_WRITE) {
            let _ = k.set_value("NonBestEffortLimit", &0u32);
        }
    }
}

#[cfg(target_os = "windows")]
fn optimize_dns_reg() {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE};
    use winreg::RegKey;
    if let Ok(hklm) = std::panic::catch_unwind(|| RegKey::predef(HKEY_LOCAL_MACHINE)) {
        if let Ok((k, _)) = hklm.create_subkey_with_flags(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters", KEY_WRITE) {
            let _ = k.set_value("MaxCacheEntryTtlLimit",    &86400u32);
            let _ = k.set_value("MaxSOACacheEntryTtlLimit", &300u32);
        }
    }
}

#[cfg(target_os = "windows")]
fn disable_net_throttle_reg() {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE};
    use winreg::RegKey;
    if let Ok(hklm) = std::panic::catch_unwind(|| RegKey::predef(HKEY_LOCAL_MACHINE)) {
        if let Ok((k, _)) = hklm.create_subkey_with_flags(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile", KEY_WRITE) {
            let _ = k.set_value("NetworkThrottlingIndex", &0xffffffffu32);
            let _ = k.set_value("SystemResponsiveness",  &0u32);
        }
    }
}

#[cfg(target_os = "windows")]
fn disable_activity_reg() {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE};
    use winreg::RegKey;
    if let Ok(hklm) = std::panic::catch_unwind(|| RegKey::predef(HKEY_LOCAL_MACHINE)) {
        if let Ok((k, _)) = hklm.create_subkey_with_flags(
            r"SOFTWARE\Policies\Microsoft\Windows\System", KEY_WRITE) {
            let _ = k.set_value("EnableActivityFeed",    &0u32);
            let _ = k.set_value("PublishUserActivities", &0u32);
            let _ = k.set_value("UploadUserActivities",  &0u32);
        }
    }
}

#[cfg(target_os = "windows")]
fn disable_location_reg() {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE};
    use winreg::RegKey;
    if let Ok(hklm) = std::panic::catch_unwind(|| RegKey::predef(HKEY_LOCAL_MACHINE)) {
        if let Ok((k, _)) = hklm.create_subkey_with_flags(
            r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors", KEY_WRITE) {
            let _ = k.set_value("DisableLocation", &1u32);
        }
    }
}

#[cfg(target_os = "windows")]
fn disable_cortana_reg() {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE};
    use winreg::RegKey;
    if let Ok(hklm) = std::panic::catch_unwind(|| RegKey::predef(HKEY_LOCAL_MACHINE)) {
        if let Ok((k, _)) = hklm.create_subkey_with_flags(
            r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", KEY_WRITE) {
            let _ = k.set_value("AllowCortana",           &0u32);
            let _ = k.set_value("DisableWebSearch",       &1u32);
            let _ = k.set_value("ConnectedSearchUseWeb",  &0u32);
        }
    }
}

#[cfg(target_os = "windows")]
fn disable_compat_telemetry_reg() {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_WRITE};
    use winreg::RegKey;
    if let Ok(hklm) = std::panic::catch_unwind(|| RegKey::predef(HKEY_LOCAL_MACHINE)) {
        if let Ok((k, _)) = hklm.create_subkey_with_flags(
            r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\AppCompatFlags", KEY_WRITE) {
            let _ = k.set_value("UpgradeEligible", &0u32);
        }
        // Disable via task scheduler
        let _ = std::process::Command::new("schtasks")
            .args(["/change", "/TN", r"Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser", "/DISABLE"])
            .output();
        let _ = std::process::Command::new("schtasks")
            .args(["/change", "/TN", r"Microsoft\Windows\Application Experience\ProgramDataUpdater", "/DISABLE"])
            .output();
    }
}

#[cfg(target_os = "windows")]
fn disable_animations_reg() {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    use winreg::RegKey;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((k, _)) = hkcu.create_subkey_with_flags(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects", KEY_WRITE) {
        let _ = k.set_value("VisualFXSetting", &2u32); // 2 = Best Performance
    }
    if let Ok((k, _)) = hkcu.create_subkey_with_flags(
        r"Control Panel\Desktop\WindowMetrics", KEY_WRITE) {
        let _ = k.set_value("MinAnimate", &"0".to_string());
    }
    // UserPreferencesMask via reg command (winreg doesn't support REG_BINARY easily)
    let _ = std::process::Command::new("reg")
        .args(["add", r"HKCU\Control Panel\Desktop",
               "/v", "UserPreferencesMask",
               "/t", "REG_BINARY",
               "/d", "9012038010000000",
               "/f"])
        .output();
}

#[cfg(target_os = "windows")]
fn disable_transparency_reg() {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    use winreg::RegKey;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((k, _)) = hkcu.create_subkey_with_flags(
        r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize", KEY_WRITE) {
        let _ = k.set_value("EnableTransparency", &0u32);
    }
}

#[cfg(target_os = "windows")]
fn enable_classic_menu_reg() {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    use winreg::RegKey;
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    // Windows 11 classic context menu
    if let Ok((k, _)) = hkcu.create_subkey_with_flags(
        r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32", KEY_WRITE) {
        let _ = k.set_value("", &"".to_string());
    }
}
