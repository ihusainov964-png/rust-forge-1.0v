// core/persistence.rs — Save/load app config to disk

use crate::config::AppConfig;
use anyhow::{anyhow, Result};
use log::{info, warn};
use std::path::PathBuf;

fn config_path() -> Result<PathBuf> {
    let dir = dirs::config_dir()
        .ok_or_else(|| anyhow!("Cannot find config directory"))?
        .join("RustForge");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("config.json"))
}

pub fn load_config() -> AppConfig {
    match try_load_config() {
        Ok(cfg) => {
            info!("Config loaded");
            cfg
        }
        Err(e) => {
            warn!("Config load failed ({}), using defaults", e);
            AppConfig::default()
        }
    }
}

fn try_load_config() -> Result<AppConfig> {
    let path = config_path()?;
    if !path.exists() {
        return Err(anyhow!("Config file does not exist"));
    }
    let content = std::fs::read_to_string(&path)?;
    let cfg: AppConfig = serde_json::from_str(&content)?;
    Ok(cfg)
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let path = config_path()?;
    let content = serde_json::to_string_pretty(config)?;
    std::fs::write(&path, content)?;
    info!("Config saved to {:?}", path);
    Ok(())
}

pub fn get_log_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("RustForge")
        .join("rustforge.log")
}
