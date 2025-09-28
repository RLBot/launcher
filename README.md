# RLBot GUI Launcher

A launcher for [RLBot v5](https://rlbot.org/v5).

Installs and updates [rlbotgui.exe](https://github.com/RLBot/gui) and [RLBotServer.exe](https://github.com/RLBot/core) in the `%localappdata%/RLBot5` folder. 
Updates itself automatically, too.

## Compiling

### Windows

Make sure you have the [rust toolchain installed](https://rustup.rs/). Build using `cargo build --release`

### Cross-compiling on linux

Make sure you have the [rust toolchain](https://rustup.rs/) and [cargo-xwin](https://github.com/rust-cross/cargo-xwin) installed. Build using `cargo xwin build --release`
