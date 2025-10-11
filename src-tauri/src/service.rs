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
    // Init default instances.
    // Only STUN/TURN servers.
    let config = Config {
        turms_url,
        rtc: vec![IceServer {
            urls: vec!["stun:stun.l.google.com:19302".into()],
            ..Default::default()
        }],
    };
    let yamlconfig =
        serde_yaml::to_string(&config).map_err(|e| e.to_string())?;
    let turms = Turms::from_config(ConfigFinder::<String>::Text(yamlconfig))
        .map_err(|e| e.to_string())?;
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
                .map_err(|_| "user not created".to_string())?;
            state.lock().await.user = Some(user);
        },
    };

    Ok(())
}

/// Get a [`User`].
/// If no `id` is specified, get current user.
#[tauri::command]
pub async fn get_user(
    state: State<'_>,
    id: Option<String>,
) -> Result<User, String> {
    match id {
        Some(id) => Ok(state
            .lock()
            .await
            .database
            .get_user(crate::database::Get::Id(id))
            .map_err(|_| "no user".to_string())?
            .0),
        None => {
            if let Some(user) = state.lock().await.user.clone() {
                Ok(user)
            } else {
                Err("no user".into())
            }
        },
    }
}

/// Get all conversations.
#[tauri::command]
pub async fn get_conversations(state: State<'_>) -> Result<Vec<User>, String> {
    state
        .lock()
        .await
        .database
        .get_conversations()
        .map_err(|_| "failed to get conversations".to_string())
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
