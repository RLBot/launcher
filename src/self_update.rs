use std::{env, fs, io::Read, path::Path};

use crate::github::Release;
use crate::pause;
use anyhow::Context;
use tracing::{info, warn};
use yansi::Paint;

const RLBOT_LAUNCHER_REPO_NAME: &str = "launcher-v5";

fn self_update(new_release: &Release) -> anyhow::Result<()> {
    let asset = new_release
        .assets
        .iter()
        .find(|a| a.name.contains("launcher.exe"))
        .context("Could not find binary in latest release of launcher")?;

    info!("Downloading latest release of launcher...");

    let response = ureq::get(&asset.browser_download_url).call()?;

    let mut bytes = Vec::new();
    response.into_body().into_reader().read_to_end(&mut bytes)?;

    info!("Updating self... DO NOT CLOSE THIS WINDOW");
    let temp_bin = Path::join(env::temp_dir().as_path(), "TEMPlauncher.exe");
    fs::write(&temp_bin, bytes)?;
    self_replace::self_replace(&temp_bin)?;
    fs::remove_file(temp_bin)?;
    info!(
        "Done! {}",
        "Please restart this program to continue.".yellow()
    );
    pause();

    Ok(())
}

pub fn check_self_update(force_update: bool) -> anyhow::Result<bool> {
    let latest_release_url =
        format!("https://api.github.com/repos/RLBot/{RLBOT_LAUNCHER_REPO_NAME}/releases/latest");
    let Ok(req) = ureq::get(&latest_release_url)
        .header("User-Agent", "rlbot-launcher-v5")
        .call()
    else {
        warn!("Self-update not available: Could not find latest release");
        return Ok(false);
    };

    let req_text = &req.into_body().read_to_string()?;

    let Ok(latest_release) = serde_json::from_str::<Release>(req_text) else {
        warn!("Self-update not available: Could not parse latest release");
        return Ok(false);
    };

    let current_version_name = env!("CARGO_PKG_VERSION");
    let latest_version_name = &latest_release.name;

    if current_version_name != latest_version_name {
        info!("Update found, self-updating...");
        self_update(&latest_release)?;
        return Ok(true);
    } else if force_update {
        info!("Forcing self-update...");
        self_update(&latest_release)?;
        return Ok(true);
    }

    info!("Already using latest version!");
    Ok(false)
}
