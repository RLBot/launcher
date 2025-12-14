mod github;
mod self_update;

use crate::github::Release;
use anyhow::anyhow;
use clap::Parser;
use console::Term;
use directories::BaseDirs;
use self_update::check_self_update;
use std::process::Command;
use std::{
    env, fs,
    io::{Read, Write, stdout},
    net::TcpStream,
    path::{Path, PathBuf},
};
use tracing::{error, info, warn};
use yansi::Paint;

/*
File structure of RLBot v5:
```
%localappdata%/
  RLBot5/
    bin/
      RLBotServer.exe
      RLBotGUI.exe
    bots/
      botpack/
      local/    # bots created through the gui
```
 */

const RLBOT_BIN_DIR: &str = "RLBot5/bin";
const RLBOT_GUI_BIN_NAME: &str = "rlbotgui.exe";
const RLBOT_SERVER_BIN_NAME: &str = "RLBotServer.exe";

// github redirects to new repo name/location if this updates
const RLBOT_GUI_REPO_NAME: &str = "gui";
const RLBOT_SERVER_REPO_NAME: &str = "core";

/// Launcher for RLBotGUI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Force update the launcher
    #[arg(short, long, default_value_t = false)]
    force_update_self: bool,

    /// Force update the gui
    #[arg(short = 'g', long, default_value_t = false)]
    force_update_gui: bool,

    /// Force update the server
    #[arg(short = 's', long, default_value_t = false)]
    force_update_server: bool,

    /// Run as if offline
    #[arg(short, long, default_value_t = false)]
    offline: bool,
}

fn realmain() -> anyhow::Result<()> {
    let args = Args::parse();
    let rlbot_ascii_art = include_str!("../assets/rlbot-ascii-art.txt");
    println!("{}\n", rlbot_ascii_art.green());

    info!("Checking for internet connection...");

    let is_online = !args.offline && is_online();

    info!("Is online: {is_online}");

    // Check for self update
    if is_online {
        info!("Checking for self-updates...");
        let self_updated = check_self_update(args.force_update_self).unwrap_or_else(|e| {
            error!("{}", e.to_string());
            warn!(
                "Self-update failed due to previous error. Skipping self-update and running anyway"
            );
            false
        });

        if self_updated {
            return Ok(());
        }
    } else {
        warn!("Not checking for updates because no internet connection was found");
    }

    let base_dirs = BaseDirs::new().ok_or(anyhow!("Could not get BaseDirs"))?;

    // Check for RLBot5 path
    let rlbot_bin_dir = Path::join(base_dirs.data_local_dir(), RLBOT_BIN_DIR);
    if !rlbot_bin_dir.exists() {
        fs::create_dir_all(rlbot_bin_dir.clone())?;
    }

    if is_online {
        // Update binaries
        if let Err(e) = update_binary(
            rlbot_bin_dir.clone(),
            RLBOT_GUI_BIN_NAME,
            RLBOT_GUI_REPO_NAME,
            args.force_update_gui,
        ) {
            error!("{}", e.to_string());
        }
        if let Err(e) = update_binary(
            rlbot_bin_dir.clone(),
            RLBOT_SERVER_BIN_NAME,
            RLBOT_SERVER_REPO_NAME,
            args.force_update_server,
        ) {
            error!("{}", e.to_string());
        }
    }

    // Run RLBot server and gui
    let server_path = rlbot_bin_dir.clone().join(RLBOT_SERVER_BIN_NAME);
    let mut server_process = Command::new(server_path)
        .current_dir(env::temp_dir())
        .spawn()?;

    let gui_path = rlbot_bin_dir.join(RLBOT_GUI_BIN_NAME);
    let exit_status = Command::new(gui_path)
        .current_dir(env::temp_dir())
        .status()?; // Blocking
    if !exit_status.success() {
        Err(anyhow!("Command failed"))?;
    }

    server_process.kill()?;

    Ok(())
}

fn update_binary(
    rlbot_bin_dir: PathBuf,
    bin_name: &str,
    repo_name: &str,
    force: bool,
) -> Result<bool, anyhow::Error> {
    // Get sha of local bin, if any
    let bin_path = rlbot_bin_dir.join(bin_name);
    let local_sha = fs::read(bin_path.clone())
        .map(|bytes| sha256::digest(&bytes))
        .ok();

    // Get sha from latest GitHub release
    let latest_release_url =
        format!("https://api.github.com/repos/RLBot/{repo_name}/releases/latest");
    let req = ureq::get(&latest_release_url)
        .header("User-Agent", "rlbot-gui-launcher")
        .call()
        .map_err(|e| anyhow!("Could not get latest release of RLBot/{}: {}", repo_name, e))?;

    let req_text = &req.into_body().read_to_string()?;

    let latest_release = serde_json::from_str::<Release>(req_text).map_err(|e| {
        anyhow!(
            "Could not parse latest release of RLBot/{}: {}",
            repo_name,
            e
        )
    })?;

    let asset = latest_release
        .assets
        .iter()
        .find(|a| a.name == bin_name)
        .ok_or(anyhow!(
            "Could not find {} asset in latest release",
            bin_name
        ))?;

    let asset_sha = asset
        .digest
        .split(':')
        .skip(1)
        .next()
        .expect("GitHub digest starts with 'sha256:'");

    // If sha is the same, we are up to date
    if let Some(ref sha) = local_sha
        && asset_sha == *sha
        && !force
    {
        info!("{} is up to date", bin_name);
        return Ok(false);
    } else if force {
        info!("Forcing update of {} ...", bin_name);
    }

    // Download and replace bin
    info!(
        "Downloading latest {} ({})...",
        bin_name, latest_release.name
    );
    let response = ureq::get(&asset.browser_download_url).call()?;

    info!("Applying update to {} ...", bin_name);
    let mut bytes = Vec::new();
    response.into_body().into_reader().read_to_end(&mut bytes)?;

    fs::write(&bin_path, bytes)?;

    Ok(true)
}

fn is_online() -> bool {
    TcpStream::connect("github.com:80").is_ok()
}

fn pause() {
    print!("Press any key to exit... ");
    stdout().flush().expect("could not flush stdout");

    let term = Term::stdout();
    term.read_key().expect("failed to read key");
}

#[cfg(not(windows))]
compile_error!("Only windows is supported");

fn main() {
    tracing_subscriber::fmt::init();

    if let Err(e) = realmain() {
        error!("{}", e.to_string());
        info!("If you need help, join our discord! https://rlbot.org/discord/");
        pause();
    }
}
