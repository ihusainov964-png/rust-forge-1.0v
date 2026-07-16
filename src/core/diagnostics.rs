// core/diagnostics.rs — READ-ONLY system diagnostics. Nothing here writes
// to the registry, kills a service, or changes a setting. The goal is to
// answer "why is FPS what it is" before touching anything else, which is
// exactly the step that was skipped last time.

use anyhow::Result;
use std::process::Command;

#[derive(Debug, Clone, Default)]
pub struct DiagnosticReport {
    pub active_power_plan: String,
    pub is_ultimate_or_high_perf: bool,
    pub core_parking_min_percent: Option<u32>,
    pub game_bar_enabled: Option<bool>,
    pub hags_enabled: Option<bool>,          // Hardware-accelerated GPU scheduling
    pub fullscreen_opt_hint: String,
    pub gpu_driver_date_hint: String,
    pub warnings: Vec<String>,
}

pub fn run_diagnostics() -> Result<DiagnosticReport> {
    let mut report = DiagnosticReport::default();

    #[cfg(target_os = "windows")]
    {
        // Active power plan — parse `powercfg /getactivescheme`
        if let Ok(out) = Command::new("powercfg").arg("/getactivescheme").output() {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            report.is_ultimate_or_high_perf =
                s.to_lowercase().contains("ultimate") || s.to_lowercase().contains("high performance");
            report.active_power_plan = s;
            if !report.is_ultimate_or_high_perf {
                report.warnings.push(
                    "⚠️ Активная схема питания — не Ultimate/High Performance. \
                     Это первое, что стоит проверить перед чем-либо ещё.".into()
                );
            }
        }

        // Core parking minimum — GUID 0cc5b647-c1df-4637-891a-dec35c318583
        // is Microsoft's own documented core-parking sub-setting. Reading
        // it back tells us if it's ALREADY effectively disabled by the
        // Ultimate Performance plan (it usually is) before we'd even
        // consider touching it ourselves.
        if let Ok(out) = Command::new("powercfg")
            .args(["/query", "SCHEME_CURRENT", "SUB_PROCESSOR", "0cc5b647-c1df-4637-891a-dec35c318583"])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout);
            report.core_parking_min_percent = parse_current_ac_value(&s);
            if let Some(pct) = report.core_parking_min_percent {
                if pct >= 100 {
                    report.warnings.push("✅ Core parking уже эффективно отключён текущей схемой питания.".into());
                } else {
                    report.warnings.push(format!(
                        "ℹ️ Core parking min = {}%. Ядра могут «засыпать» под низкой нагрузкой.", pct
                    ));
                }
            }
        }

        // Game Bar status (read-only, we already have a toggle for this —
        // this just confirms whether it's currently on)
        report.game_bar_enabled = query_reg_dword(
            r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR", "AppCaptureEnabled"
        ).map(|v| v != 0);

        // HAGS — HKLM requires elevated read in some configs; best-effort
        report.hags_enabled = query_reg_dword(
            r"HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers", "HwSchMode"
        ).map(|v| v == 2);

        report.fullscreen_opt_hint =
            "Это свойство .exe-файла Rust, реестром не читается надёжно — проверь вручную: \
             ПКМ на RustClient.exe → Свойства → Совместимость → \
             «Отключить оптимизацию для полноэкранного режима».".to_string();
    }

    #[cfg(not(target_os = "windows"))]
    {
        report.warnings.push("Диагностика доступна только на Windows.".into());
    }

    Ok(report)
}

#[cfg(target_os = "windows")]
fn parse_current_ac_value(powercfg_output: &str) -> Option<u32> {
    // powercfg /query output has a line like:
    // "Current AC Power Setting Index: 0x00000064"
    for line in powercfg_output.lines() {
        if line.contains("Current AC Power Setting Index") {
            if let Some(hex) = line.split(':').nth(1) {
                let hex = hex.trim().trim_start_matches("0x");
                if let Ok(v) = u32::from_str_radix(hex, 16) {
                    return Some(v);
                }
            }
        }
    }
    None
}

#[cfg(target_os = "windows")]
fn query_reg_dword(key_path: &str, value_name: &str) -> Option<u32> {
    use winreg::enums::*;
    use winreg::RegKey;
    let (hive, subkey) = key_path.split_once('\\')?;
    let root = match hive {
        "HKCU" => RegKey::predef(HKEY_CURRENT_USER),
        "HKLM" => RegKey::predef(HKEY_LOCAL_MACHINE),
        _ => return None,
    };
    root.open_subkey_with_flags(subkey, KEY_READ).ok()?
        .get_value::<u32, _>(value_name).ok()
}
