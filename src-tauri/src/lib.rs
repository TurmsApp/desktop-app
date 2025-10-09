//! Tauri stuff.

mod database;
pub mod models;
pub(crate) mod service;

use anyhow::Result;
use keyring::Entry;
use libturms::Turms;
use rand::{Rng, TryRngCore};
use tauri::Manager;
use tauri::async_runtime::Mutex;

use std::path::PathBuf;

use crate::database::Database;

#[derive(Debug)]
pub(crate) struct State {
    /// Handle discovery, p2p, crypto, etc.
    pub turms: Option<Turms>,
    pub user: Option<models::user::User>,
    pub(crate) database: Database,
}

fn init_state(path: PathBuf) -> Result<State> {
    // Get security key to decrypt database.
    let entry = Entry::new("turms", "key")?;
    let key = match entry.get_secret() {
        Ok(key) => key,
        Err(_) => {
            // Generate a random 256 bits key.
            use rand::SeedableRng;
            use rand::rngs::OsRng;
            use rand_chacha::ChaCha20Rng;

            let mut seed = [0u8; 32];
            let _ = OsRng.try_fill_bytes(&mut seed);

            let mut key = [0u8; 32];
            let mut rng = ChaCha20Rng::from_seed(seed);
            rng.fill(&mut key[..]);
            let key = key.to_vec();
            entry.set_secret(&key).expect("cannot save secure key");
            key
        }
    };

    // Init database.
    let database = Database::new(path.join("encrypted.db3"), hex::encode(key))
        .expect("cannot create database");
    database.create_tables()?;

    let mut state = State {
        turms: None,
        user: None,
        database,
    };

    // If previously connected, reconnect.
    if let Ok((user, config)) = state.database.get_user(database::Get::Me) {
        state.user = Some(user);
        let config = serde_yaml::to_string(&config.unwrap())
            .map_err(|e| e.to_string())
            .unwrap();
        state.turms =
            Some(Turms::from_config(libturms::ConfigFinder::<String>::Text(config)).unwrap());
    }

    Ok(state)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            service::init,
            service::get_user,
            service::generate_offer
        ])
        .setup(|app| {
            let path = if cfg!(debug_assertions) {
                app.path().app_cache_dir()
            } else {
                app.path().app_data_dir()
            };

            // Crash if secure boot is not guaranteed.
            let state = init_state(path?).expect("secure boot failed");

            app.manage(Mutex::new(state));

            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::{NSVisualEffectMaterial, apply_vibrancy};
                apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)?;
            }
            #[cfg(target_os = "windows")]
            let _ = window_vibrancy::apply_mica(&window, None);

            #[cfg(debug_assertions)]
            window.open_devtools();

            window.show()?;
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
