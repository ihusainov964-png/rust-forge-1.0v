// core/monitor.rs — Real-time system monitoring via `sysinfo`.
//
// This replaces the old wmic-based static snapshot in hardware.rs for
// anything that needs to be LIVE (RAM/CPU %). hardware.rs's one-shot
// detection (CPU model name, GPU name) is still fine as-is and untouched.
//
// Design note: sysinfo needs a `refresh()` call before every read because
// most values are computed as a diff against the previous sample — keep
// one long-lived `SystemMonitor` around (already wired into RustForgeApp
// below) instead of constructing `System::new_all()` every frame, which
// is expensive.

use sysinfo::{System, SystemExt, CpuExt, PidExt, ProcessExt};

pub struct SystemMonitor {
    sys: System,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LiveStats {
    pub ram_total_mb: u64,
    pub ram_used_mb: u64,
    pub ram_percent: f32,
    pub cpu_percent: f32,
    /// Per-core usage, for a small bar-chart in the UI
    pub cpu_per_core: [f32; 32],
    pub cpu_core_count: usize,
    /// CPU usage of the Rust game process specifically, if it's running
    pub rust_process_cpu: Option<f32>,
    pub rust_process_ram_mb: Option<u64>,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    /// Call this roughly once a second (see app.rs wiring) — sysinfo's own
    /// docs recommend not refreshing on every UI frame, both because CPU
    /// usage needs a real time delta to mean anything and because a full
    /// process-list refresh isn't free.
    pub fn refresh(&mut self) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
        self.sys.refresh_processes();
    }

    pub fn snapshot(&self) -> LiveStats {
        let ram_total_kb = self.sys.total_memory();
        let ram_used_kb = self.sys.used_memory();
        let ram_percent = if ram_total_kb > 0 {
            (ram_used_kb as f32 / ram_total_kb as f32) * 100.0
        } else {
            0.0
        };

        let cpus = self.sys.cpus();
        let cpu_percent = if !cpus.is_empty() {
            cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32
        } else {
            0.0
        };

        let mut cpu_per_core = [0.0f32; 32];
        for (i, c) in cpus.iter().take(32).enumerate() {
            cpu_per_core[i] = c.cpu_usage();
        }

        // Find RustClient.exe / RustClient among running processes
        let mut rust_process_cpu = None;
        let mut rust_process_ram_mb = None;
        for (_pid, proc_) in self.sys.processes() {
            let name = proc_.name().to_lowercase();
            if name.contains("rustclient") || name == "rust.exe" {
                rust_process_cpu = Some(proc_.cpu_usage());
                rust_process_ram_mb = Some(proc_.memory() / 1024); // KB -> MB
                break;
            }
        }

        LiveStats {
            // sysinfo (0.29.x) reports memory in KiB — convert to MiB for display.
            ram_total_mb: ram_total_kb / 1024,
            ram_used_mb: ram_used_kb / 1024,
            ram_percent,
            cpu_percent,
            cpu_per_core,
            cpu_core_count: cpus.len(),
            rust_process_cpu,
            rust_process_ram_mb,
        }
    }

    /// Is Rust currently running? Cheap check used to switch button labels
    /// like "Launch" vs "Rust is already running".
    pub fn is_rust_running(&self) -> bool {
        self.rust_pid().is_some()
    }

    /// PID of the running Rust game process, if any — used to target
    /// SetPriorityClass at exactly the right process.
    pub fn rust_pid(&self) -> Option<u32> {
        self.sys.processes().iter().find_map(|(pid, p)| {
            let n = p.name().to_lowercase();
            if n.contains("rustclient") || n == "rust.exe" {
                Some(pid.as_u32())
            } else {
                None
            }
        })
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}
