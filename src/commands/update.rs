use anyhow::{bail, Context, Result};
use std::env;
use std::process::Command;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const INSTALL_URL: &str = "https://sgit.vercel.app/install.sh";
const GITHUB_API_URL: &str =
    "https://api.github.com/repos/ThomasNowProductions/SGIT/releases/latest";
const UPDATE_CHECK_INTERVAL_SECS: u64 = 24 * 60 * 60;

fn get_current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn get_last_check_file() -> Option<std::path::PathBuf> {
    dirs::cache_dir().map(|p| p.join("sgit").join("last_update_check"))
}

fn get_time_since_last_check() -> Option<Duration> {
    let path = get_last_check_file()?;
    let contents = std::fs::read_to_string(path).ok()?;
    let timestamp: u64 = contents.trim().parse().ok()?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    Some(Duration::from_secs(now.saturating_sub(timestamp)))
}

fn record_update_check() {
    if let Some(path) = get_last_check_file() {
        let _ = std::fs::create_dir_all(path.parent().unwrap());
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs().to_string())
            .unwrap_or_default();
        let _ = std::fs::write(&path, now);
    }
}

fn fetch_latest_version() -> Result<String> {
    let output = Command::new("curl")
        .args([
            "-fsSL",
            "-H",
            "Accept: application/vnd.github.v3+json",
            GITHUB_API_URL,
        ])
        .output()
        .context("Failed to fetch release info from GitHub")?;

    if !output.status.success() {
        bail!(
            "Failed to fetch release info: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let response = String::from_utf8_lossy(&output.stdout);
    for line in response.lines() {
        let line = line.trim();
        if line.starts_with("\"tag_name\":") {
            let version = line
                .split(':')
                .nth(1)
                .unwrap_or("")
                .trim()
                .trim_matches('"')
                .trim_start_matches('v')
                .trim_matches(',');
            return Ok(version.to_string());
        }
    }

    bail!("Could not parse version from GitHub response")
}

fn version_is_newer(latest: &str, current: &str) -> bool {
    let parse_version =
        |v: &str| -> Vec<u32> { v.split('.').filter_map(|s| s.parse().ok()).collect() };

    let latest_parts = parse_version(latest);
    let current_parts = parse_version(current);

    for i in 0..latest_parts.len().max(current_parts.len()) {
        let l = latest_parts.get(i).unwrap_or(&0);
        let c = current_parts.get(i).unwrap_or(&0);
        if l > c {
            return true;
        }
        if l < c {
            return false;
        }
    }
    false
}

pub fn check_and_auto_update() -> Result<()> {
    if env::var("SGIT_SKIP_UPDATE_CHECK").is_ok() {
        return Ok(());
    }

    if let Some(elapsed) = get_time_since_last_check()
        && elapsed.as_secs() < UPDATE_CHECK_INTERVAL_SECS
    {
        return Ok(());
    }

    let latest = match fetch_latest_version() {
        Ok(v) => v,
        Err(_) => return Ok(()),
    };

    record_update_check();

    let current = get_current_version();
    if !version_is_newer(&latest, current) {
        return Ok(());
    }

    println!("Updating sgit from v{} to v{}...", current, latest);

    let mut sh = Command::new("sh");
    sh.arg("-c")
        .arg("curl -fsSL https://sgit.vercel.app/install.sh | sh");

    let status = sh.status().context("Failed to run installer")?;

    if status.success() {
        println!("✓ Updated to v{}", latest);
    }

    Ok(())
}

pub fn run_self_update(target_version: Option<&str>) -> Result<()> {
    println!("Current version: v{}", get_current_version());
    println!("Running installer...");

    let curl = Command::new("curl")
        .args(["-fsSL", INSTALL_URL])
        .output()
        .context("Failed to run curl. Is curl installed?")?;

    if !curl.status.success() {
        bail!(
            "Failed to download install script: {}",
            String::from_utf8_lossy(&curl.stderr)
        );
    }

    let mut sh = Command::new("sh");
    sh.arg("-c")
        .arg("curl -fsSL https://sgit.vercel.app/install.sh | sh");

    if let Some(v) = target_version {
        let version = if v.starts_with('v') {
            v.to_string()
        } else {
            format!("v{}", v)
        };
        sh.env("SGIT_VERSION", version);
    }

    let status = sh.status().context("Failed to run installer")?;

    if !status.success() {
        bail!("Installer failed with exit code {:?}", status.code());
    }

    Ok(())
}
