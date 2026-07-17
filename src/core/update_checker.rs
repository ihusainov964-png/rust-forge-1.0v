// core/update_checker.rs — checks GitHub Releases for a newer version.
// Pure network read of a public API, nothing touches the system. Runs on
// a background thread so it can never freeze the UI (learned that lesson
// with the priority-boost function already).

use serde::Deserialize;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

const REPO_OWNER: &str = "ihusainov964-png";
const REPO_NAME: &str = "rust-forge-1.0v";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub latest_version: String,
    pub download_url: String,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
    #[serde(default)]
    draft: bool,
    #[serde(default)]
    prerelease: bool,
}

/// Kick off the check on a background thread. Returns a Receiver you can
/// poll (non-blockingly) from the egui update loop — see app.rs wiring.
pub fn check_for_updates_async() -> Receiver<Option<UpdateInfo>> {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let result = check_now().ok().flatten();
        let _ = tx.send(result);
    });

    rx
}

fn check_now() -> anyhow::Result<Option<UpdateInfo>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        REPO_OWNER, REPO_NAME
    );

    // GitHub's API requires a User-Agent header or it 403s every request.
    let response: GithubRelease = ureq::get(&url)
        .set("User-Agent", "RustForge-UpdateChecker")
        .set("Accept", "application/vnd.github+json")
        .timeout(std::time::Duration::from_secs(5))
        .call()?
        .into_json()?;

    if response.draft || response.prerelease {
        return Ok(None);
    }

    let latest = response.tag_name.trim_start_matches('v');
    let current = CURRENT_VERSION;

    let is_newer = match (semver::Version::parse(latest), semver::Version::parse(current)) {
        (Ok(l), Ok(c)) => l > c,
        // If either version string doesn't parse as semver, fall back to a
        // plain string comparison rather than silently assuming "no update"
        // — better to show a possibly-redundant banner than hide a real one.
        _ => latest != current,
    };

    if is_newer {
        Ok(Some(UpdateInfo {
            latest_version: response.tag_name,
            download_url: response.html_url,
        }))
    } else {
        Ok(None)
    }
}
