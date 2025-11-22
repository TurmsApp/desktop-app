//! Chat service using libturms.

use tauri::async_runtime::Mutex;

use crate::errors::Result;
use crate::errors::TurmsError;
use crate::models::user::User;

use std::sync::Arc;

pub type State<'a> = tauri::State<'a, Arc<Mutex<crate::State>>>;

/// On connect, inits Turms.
#[tauri::command]
pub async fn init(
    state: State<'_>,
    token: Option<String>,
    turms_url: Option<String>,
) -> Result<()> {
    crate::deeplink::init(state.inner(), token, turms_url).await
}

/// Get a [`User`].
/// If no `id` is specified, get current user.
#[tauri::command]
pub async fn get_user(state: State<'_>, id: Option<String>) -> Result<User> {
    match id {
        Some(id) => Ok(state
            .lock()
            .await
            .database
            .get_user(crate::database::Get::Id(id))?),
        None => {
            if let Some(user) = state.lock().await.user.clone() {
                Ok(user)
            } else {
                Err(TurmsError::UserNotExists)
            }
        },
    }
}

/// Get all conversations.
#[tauri::command]
pub async fn get_conversations(state: State<'_>) -> Result<Vec<User>> {
    state.lock().await.database.get_conversations()
}

/// Generate user WebRTC offer.
#[tauri::command]
pub async fn generate_offer(state: State<'_>) -> Result<String> {
    if let Some(offer) = state
        .lock()
        .await
        .turms
        .as_mut()
        .map(|turms| turms.create_peer_offer())
    {
        match offer.await {
            Ok(offer) => Ok(offer),
            Err(_) => Err(TurmsError::TurmsInstanceNotInitialized),
        }
    } else {
        Err(TurmsError::TurmsInstanceNotInitialized)
    }
}

/// Answer to an offer and generates WebRTC answer.
#[tauri::command]
pub async fn connect_peer(state: State<'_>, session: String) -> Result<String> {
    match state.lock().await.turms.as_mut() {
        Some(turms) => match turms.connect(&session).await? {
            libturms::Session::Offer(offer) => Ok(offer),
            libturms::Session::Answered => Ok(String::default()),
            _ => Err(TurmsError::InvalidSession),
        },
        None => Err(TurmsError::TurmsInstanceNotInitialized),
    }
}
