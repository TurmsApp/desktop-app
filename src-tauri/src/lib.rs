//! Tauri stuff.

mod database;
pub mod models;
pub(crate) mod service;

use keyring::Entry;
use libturms::Turms;
use rand::{Rng, TryRngCore};
use tauri::Manager;
use tokio::sync::Mutex;

use crate::database::Database;
use crate::models::user::User;

#[derive(Debug)]
pub(crate) struct State {
    /// Handle discovery, p2p, crypto, etc.
    pub turms: Option<Turms>,
    pub user: Option<models::user::User>,
    pub(crate) database: Database,
}

async fn init_state() -> State {
    // Get security key to decrypt database.
    let entry = Entry::new("turms", "key").unwrap();
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
    let database =
        Database::new("./encrypted.db3", hex::encode(key)).expect("cannot create database");
    //state.database.create_tables().unwrap();

    let mut state = State {
        turms: None,
        user: None,
        database,
    };

    // If previously connected, reconnect.
    state.user = Entry::new("turms", "user_id")
        .unwrap()
        .get_password()
        .ok()
        .map(|user_id| User::new(user_id, "Guest".into()));

    // Connect to Turms instance.
    let config = libturms::Config {
        turms_url: None,
        rtc: vec![libturms::IceServer {
            urls: vec!["stun:stun.l.google.com:19302".into()],
            ..Default::default()
        }],
    };
    let config = serde_yaml::to_string(&config)
        .map_err(|e| e.to_string())
        .unwrap();
    state.turms = Some(
        Turms::from_config(libturms::ConfigFinder::<String>::Text(config))
            .await
            .unwrap(),
    );

    state
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let state = init_state().await;

    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_stronghold::Builder::new(|_pass| todo!()).build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            service::init,
            service::get_user,
            service::generate_offer
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let main_window = app.get_webview_window("main").unwrap();
                main_window.open_devtools();
            }
            app.manage(Mutex::new(state));

            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
