use std::env;
use std::path::Path;

use anyhow::{Context, Result};

use crate::git::run_git_silent;

pub fn run_clone(url: &str, directory: Option<&str>) -> Result<()> {
    println!("→ Cloning {}...", url);

    let mut args = vec!["clone", url];
    let target_dir = directory.map(String::from);

    if let Some(ref dir) = target_dir {
        args.push(dir.as_str());
    }

    run_git_silent(&args)?;

    let repo_dir = determine_repo_dir(url, target_dir.as_deref())?;

    env::set_current_dir(&repo_dir)
        .with_context(|| format!("failed to change directory to {}", repo_dir.display()))?;

    println!("✓ Clone complete");

    Ok(())
}

fn determine_repo_dir(url: &str, directory: Option<&str>) -> Result<std::path::PathBuf> {
    if let Some(dir) = directory {
        let path = Path::new(dir);
        if path.is_absolute() {
            return Ok(path.to_path_buf());
        }
        let cwd = env::current_dir().context("failed to get current directory")?;
        return Ok(cwd.join(path));
    }

    let repo_name = extract_repo_name(url)?;
    let cwd = env::current_dir().context("failed to get current directory")?;
    Ok(cwd.join(&repo_name))
}

fn extract_repo_name(url: &str) -> Result<String> {
    let url = url.trim_end_matches('/');

    let name = if url.starts_with("git@") {
        url.rsplit(':')
            .next()
            .and_then(|s| s.rsplit('/').next())
            .map(|s| s.trim_end_matches(".git").to_string())
    } else if url.starts_with("https://") || url.starts_with("http://") {
        url.rsplit('/')
            .next()
            .map(|s| s.trim_end_matches(".git").to_string())
    } else if url.starts_with("ssh://") {
        url.rsplit('/')
            .next()
            .map(|s| s.trim_end_matches(".git").to_string())
    } else {
        url.rsplit('/')
            .next()
            .map(|s| s.trim_end_matches(".git").to_string())
    };

    name.or_else(|| {
        let name = url.rsplit('/').next()?;
        Some(name.trim_end_matches(".git").to_string())
    })
    .ok_or_else(|| anyhow::anyhow!("could not determine repository name from URL"))
}
