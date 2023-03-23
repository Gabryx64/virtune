// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod player;
use player::{next, play, prev, Player, PlayerState};

use color_eyre::Result;
use std::sync::RwLock;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    color_eyre::install()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .without_time()
        .with_file(false)
        .with_target(false)
        .with_line_number(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    tauri::Builder::default()
        .manage(PlayerState(RwLock::new(Player::new()?)))
        .invoke_handler(tauri::generate_handler![next, play, prev])
        .run(tauri::generate_context!())?;

    Ok(())
}
