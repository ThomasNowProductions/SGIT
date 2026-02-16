use anyhow::{bail, Context, Result};
use std::env;
use std::process::Command;

const INSTALL_URL: &str = "https://sgit.vercel.app/install.sh";

fn get_current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
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
