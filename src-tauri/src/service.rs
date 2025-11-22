//! Chat service using libturms.

use tauri::async_runtime::Mutex;

use crate::models::user::User;

use std::sync::Arc;

pub type State<'a> = tauri::State<'a, Arc<Mutex<crate::State>>>;

/// On connect, inits Turms.
#[tauri::command]
pub async fn init(
    state: State<'_>,
    token: Option<String>,
    turms_url: Option<String>,
) -> Result<(), String> {
    crate::deeplink::init(state.inner(), token, turms_url).await
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
            .map_err(|_| "no user".to_string())?),
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

/// Generate user WebRTC offer.
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

/// Answer to an offer and generates WebRTC answer.
#[tauri::command]
pub async fn connect_peer(
    state: State<'_>,
    session: String,
) -> Result<String, String> {
    match state.lock().await.turms.as_mut() {
        Some(turms) => {
            match turms
                .connect(&session)
                .await
                .map_err(|_| "failed to answer or offer")?
            {
                libturms::Session::Offer(offer) => Ok(offer),
                libturms::Session::Answered => Ok(String::default()),
                _ => Err("given session is not a session".to_string()),
            }
        },
        None => Err("no turms".to_string()),
    }
}
