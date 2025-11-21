//! Tauri stuff.

mod database;
mod deeplink;
mod errors;
pub mod models;
pub(crate) mod service;

use anyhow::{Result, anyhow};
use keyring::{Entry, Error::NoEntry};
use libturms::discover::jwt::*;
use libturms::p2p;
use libturms::{ConfigFinder, Turms};
use rand::{Rng, TryRngCore};
use tauri::async_runtime::Mutex;
use tauri_plugin_log::log;
use tauri::{App, Manager};
use tauri_plugin_deep_link::DeepLinkExt;

use std::path::PathBuf;
use std::sync::Arc;

use crate::database::Database;

pub(crate) struct State {
    /// Handle discovery, p2p, crypto, etc.
    pub turms: Option<Turms>,
    /// Current connected user.
    pub user: Option<models::user::User>,
    /// SQLite connection manager.
    pub(crate) database: Database,
    pub token: TokenManager,
}

fn init_state(app: &mut App, path: PathBuf) -> Result<State> {
    // Get security key to decrypt database.
    let entry = Entry::new("turms", "key")
        .map_err(|_| errors::unauthorized_key(app))?;
    let key = match entry.get_secret() {
        Ok(key) => key,
        Err(NoEntry) => {
            // Generate a random 256 bits key.
            use rand::SeedableRng;
            use rand::rngs::OsRng;
            use rand_chacha::ChaCha20Rng;

            let mut seed = [0u8; 32];
            OsRng.try_fill_bytes(&mut seed)?;

            let mut key = [0u8; 32];
            let mut rng = ChaCha20Rng::from_seed(seed);
            rng.fill(&mut key[..]);
            let key = key.to_vec();
            entry.set_secret(&key).expect("cannot save secure key");
            key
        },
        Err(_) => errors::unauthorized_key(app),
    };

    // Init database.
    let db_path = path.join("encrypted.db3");
    log::info!("database loaded on {db_path:?}");
    let database = Database::new(&db_path, hex::encode(key))?;
    database
        .create_tables()
        .map_err(|_| errors::corrupted_db(app, db_path))?;

    let mut state = State {
        turms: None,
        user: None,
        database,
        token: TokenManager::new(
            None,
            Key::Text::<String>(crate::deeplink::JWT_PUBLIC_KEY.to_string()),
        )
        .map_err(|_| errors::internal_error(app))?
        .algorithm(Algorithm::ES256),
    };

    // If previously connected, reconnect.
    if let Ok(user) = state.database.get_user(database::Get::Me) {
        log::debug!("connected user is {user:?}");
        // Restore private and public key to encrypt messages.
        let account = user
            .account
            .as_ref()
            .ok_or(anyhow!("missing account entry on database"))?;
        p2p::restore_account(account)?;

        // Restore previous configuration.
        let config = serde_yaml::to_string(
            &user
                .config
                .clone()
                .ok_or(anyhow!("missing config entry on database"))?,
        )?;
        let (turms, _receiver) =
            Turms::from_config(ConfigFinder::<String>::Text(config))?;
        state.turms = Some(turms);

        state.user = Some(user);
    }

    Ok(state)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let ctx = tauri::generate_context!();
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {}));
    }

    builder
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            service::init,
            service::get_user,
            service::generate_offer,
            service::get_conversations,
        ])
        .setup(|app| {
            let path = if cfg!(debug_assertions) {
                app.path().app_cache_dir()
            } else {
                app.path().app_data_dir()
            };

            // Crash if secure boot is not guaranteed.
            let state = init_state(app, path?).expect("secure boot failed");
            let state = Arc::new(Mutex::new(state));

            app.manage(Arc::clone(&state));

            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            {
                use window_vibrancy::{NSVisualEffectMaterial, apply_vibrancy};
                apply_vibrancy(
                    &window,
                    NSVisualEffectMaterial::HudWindow,
                    None,
                    None,
                )?;
            }
            #[cfg(target_os = "windows")]
            let _ = window_vibrancy::apply_mica(&window, None);

            #[cfg(debug_assertions)]
            window.open_devtools();

            // Handle deep linking.
            #[cfg(any(target_os = "linux", all(debug_assertions, windows)))]
            app.deep_link().register_all()?;

            let state = Arc::clone(&state);
            let start_urls = app.deep_link().get_current()?;
            if let Some(urls) = start_urls {
                deeplink::handler(Arc::clone(&state), urls);
            }

            let state = Arc::clone(&state);
            app.deep_link().on_open_url(move |event| {
                deeplink::handler(Arc::clone(&state), event.urls());
            });

            window.show()?;
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
