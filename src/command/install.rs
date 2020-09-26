use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use anyhow::{anyhow, Result};
use clap::crate_name;
use raur::Package;

use crate::cli::AurCommandOptions;
use crate::command;

const AUR_BASE_URL: &str = "https://aur.archlinux.org";

pub fn install(options: &AurCommandOptions) -> Result<()> {
    if options.remove {
        let mut pacman_args = vec!["-Rs"];
        pacman_args.extend(options.packages.iter().map(String::as_str));
        return command::pacman(&pacman_args);
    }

    let cache_path = dirs::cache_dir()
        .ok_or_else(|| anyhow!("failed to find cache dir"))?
        .join(crate_name!());

    if !cache_path.exists() {
        fs::create_dir_all(&cache_path)?;
    }

    println!("Fetching package metadata...");
    let pkgs = raur::info(&options.packages)?;

    // TODO: parallelise
    for pkg in &pkgs {
        download_pkgbuild(&cache_path, pkg)?;
    }

    for pkg in &pkgs {
        makepkg(&cache_path, pkg)?;
    }

    Ok(())
}

// TODO: --rmdeps, --clean, --cleanbuild, --nodeps, --force
fn makepkg(cache_path: impl AsRef<Path>, pkg: &Package) -> Result<()> {
    println!("Building {}...", &pkg.name);
    let pkg_path = cache_path.as_ref().join(&pkg.name);
    let status = Command::new("makepkg")
        .args(&["--syncdeps", "--install"])
        .current_dir(&pkg_path)
        .spawn()
        .map_err(|e| anyhow!("Error running makepkg: {}", e))?
        .wait()?;
    if !status.success() {
        return Err(anyhow!("Error building {}", &pkg.name));
    }

    Ok(())
}

fn download_pkgbuild(cache_path: impl AsRef<Path>, pkg: &Package) -> Result<()> {
    let pkg_path = cache_path.as_ref().join(&pkg.name);
    let git_path = pkg_path.join(".git");
    if git_path.is_dir() {
        println!("Updating {}...", &pkg.name);
        let Output { status, stderr, .. } = Command::new("git")
            .arg("fetch")
            .current_dir(pkg_path)
            .output()
            .map_err(|e| anyhow!("Error running git: {}", e))?;
        if !status.success() {
            return Err(anyhow!(
                "Error fetching {}: {}",
                &pkg.name,
                String::from_utf8_lossy(&stderr)
            ));
        }
    } else {
        let git_url = format!("{}/{}.git", AUR_BASE_URL, pkg.name);
        println!("Cloning {}...", &git_url);
        let Output { status, stderr, .. } = Command::new("git")
            .args(&["clone", "--no-progress", &git_url, &pkg.name])
            .current_dir(cache_path)
            .output()
            .map_err(|e| anyhow!("Error running git: {}", e))?;
        if !status.success() {
            return Err(anyhow!(
                "Error cloning {}: {}",
                &pkg.name,
                String::from_utf8_lossy(&stderr)
            ));
        }
    }

    Ok(())
}
