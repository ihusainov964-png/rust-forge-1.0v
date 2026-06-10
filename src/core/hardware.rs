// core/hardware.rs — Cross-platform hardware detection

use crate::config::HardwareInfo;
use log::warn;

/// Detect system hardware info
pub fn detect_hardware() -> HardwareInfo {
    let ram_total_mb = detect_ram_mb();
    let cpu_cores = detect_cpu_cores();
    let cpu_name = detect_cpu_name();
    let gpu_name = detect_gpu_name();
    let os_version = detect_os_version();
    let is_nvidia = gpu_name.to_lowercase().contains("nvidia") || gpu_name.to_lowercase().contains("geforce") || gpu_name.to_lowercase().contains("rtx") || gpu_name.to_lowercase().contains("gtx");
    let is_amd = gpu_name.to_lowercase().contains("amd") || gpu_name.to_lowercase().contains("radeon") || gpu_name.to_lowercase().contains("rx ");

    HardwareInfo {
        ram_total_mb,
        cpu_cores,
        cpu_name,
        gpu_name,
        os_version,
        is_nvidia,
        is_amd,
    }
}

fn detect_ram_mb() -> u64 {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("wmic")
            .args(["ComputerSystem", "get", "TotalPhysicalMemory", "/value"])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout);
            for line in s.lines() {
                if let Some(val) = line.strip_prefix("TotalPhysicalMemory=") {
                    if let Ok(bytes) = val.trim().parse::<u64>() {
                        return bytes / 1_048_576;
                    }
                }
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(kb) = parts[1].parse::<u64>() {
                            return kb / 1024;
                        }
                    }
                }
            }
        }
    }
    warn!("Could not detect RAM, defaulting to 8192 MB");
    8192
}

fn detect_cpu_cores() -> u32 {
    std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(4)
}

fn detect_cpu_name() -> String {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("wmic")
            .args(["cpu", "get", "Name", "/value"])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout);
            for line in s.lines() {
                if let Some(val) = line.strip_prefix("Name=") {
                    let name = val.trim().to_string();
                    if !name.is_empty() {
                        return name;
                    }
                }
            }
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("model name") {
                    if let Some(val) = line.split(':').nth(1) {
                        return val.trim().to_string();
                    }
                }
            }
        }
    }
    "Unknown CPU".to_string()
}

fn detect_gpu_name() -> String {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("wmic")
            .args(["path", "win32_videocontroller", "get", "name", "/value"])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout);
            for line in s.lines() {
                if let Some(val) = line.strip_prefix("Name=") {
                    let name = val.trim().to_string();
                    if !name.is_empty() {
                        return name;
                    }
                }
            }
        }
    }
    "Unknown GPU".to_string()
}

fn detect_os_version() -> String {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("cmd")
            .args(["/c", "ver"])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout);
            return s.trim().to_string();
        }
        return "Windows".to_string();
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    return line.replace("PRETTY_NAME=", "").replace('"', "");
                }
            }
        }
        return "Linux".to_string();
    }
    #[allow(unreachable_code)]
    "Unknown OS".to_string()
}

/// Returns a tier label for the detected hardware
pub fn get_hardware_tier(hw: &HardwareInfo) -> (&'static str, egui::Color32) {
    use egui::Color32;
    let score = (hw.cpu_cores as u64) * 100 + hw.ram_total_mb / 100;
    if score > 1800 {
        ("HIGH-END", Color32::from_rgb(180, 120, 255))
    } else if score > 1000 {
        ("MID-RANGE", Color32::from_rgb(230, 160, 40))
    } else {
        ("POTATO", Color32::from_rgb(200, 60, 40))
    }
}
