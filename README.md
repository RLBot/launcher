# RLBot GUI Launcher

A launcher for [RLBot v5](https://rlbot.org/v5).

Installs and updates [rlbotgui.exe](https://github.com/RLBot/gui) and [RLBotServer.exe](https://github.com/RLBot/core) in the `%localappdata%/RLBot5` folder. 
Updates itself automatically, too.

## Compiling

### Windows

Make sure you have the [rust toolchain installed](https://rustup.rs/). Build using `cargo build --release`

### Cross-compiling on linux

Make sure you have the [rust toolchain](https://rustup.rs/) and [cargo-xwin](https://github.com/rust-cross/cargo-xwin) installed. Build using `cargo xwin build --release`

## Updating the launcher

Once the changes have been committed and the version in `Cargo.toml` has been updated, the launcher can be updated by following these steps:

1. Bump the version in `Cargo.toml`.
1. Run `cargo build --release` (be careful you don't run it as it will replace itself with the old version)
1. Create a new GitHub release and upload the exe. The release name must match the version in `Cargo.toml`.

## Updating the installer

1. Install [NSIS](https://nsis.sourceforge.io/Main_Page).
1. Right-click on `installer.nsi` and select `Compile NSIS Script` or run `makensis installer.nsi`.
1. Replace the installer file in the [Installer release](https://github.com/RLBot/launcher-v5/releases/tag/installer).

The installer technically only needs to be created once (since the launcher is self-updating),
but creating an intaller that installs the newest launcher immediately avoids new users having to restart the launcher on first run.
