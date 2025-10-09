//! Chat service using libturms.

use libturms::{Config, ConfigFinder, IceServer, Turms};
use tauri::async_runtime::Mutex;

use crate::models::user::User;

pub type State<'a> = tauri::State<'a, Mutex<crate::State>>;

/// On connect, inits Turms.
#[tauri::command]
pub async fn init(
    state: State<'_>,
    username: Option<String>,
    _password: Option<String>,
    turms_url: Option<String>,
) -> Result<(), String> {
    let config = Config {
        turms_url,
        rtc: vec![IceServer {
            urls: vec!["stun:stun.l.google.com:19302".into()],
            ..Default::default()
        }],
    };
    let yamlconfig = serde_yaml::to_string(&config).map_err(|e| e.to_string())?;

    let turms = Turms::from_config(ConfigFinder::<String>::Text(yamlconfig)).unwrap();
    state.lock().await.turms = Some(turms);

    match username {
        Some(_) => unimplemented!(),
        None => {
            let user = User::new("guest", "Guest");
            state
                .lock()
                .await
                .database
                .create_user(&user, Some(config))
                .unwrap();
            state.lock().await.user = Some(user);
        }
    };

    Ok(())
}

/// Get a [`User`].
/// If no `id` is specified, get current user.
#[tauri::command]
pub async fn get_user(state: State<'_>, _id: Option<String>) -> Result<User, String> {
    if let Some(user) = state.lock().await.user.clone() {
        Ok(user)
    } else {
        Err("no user".into())
    }
}

/// Get a [`User`].
/// If no `id` is specified, get current user.
#[tauri::command]
pub async fn generate_offer(state: State<'_>) -> Result<String, String> {
    if let Some(offer) = state
        .lock()
        .await
        .turms
        .as_mut()
        .map(|turms| turms.create_peer_offer())
    {
        match offer.await {
            Ok(offer) => Ok(offer),
            Err(_) => Err("no turms".into()),
        }
    } else {
        Err("no turms".into())
    }
}
