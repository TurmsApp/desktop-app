//! Handler for deep links.

use libturms::{Config, ConfigFinder, RTCIceServer, Turms};
use tauri::async_runtime::Mutex;
use tauri_plugin_log::log;

use std::sync::Arc;

use crate::State;
use crate::models::user::User;

const GUEST: &str = "Guest";

pub const JWT_PUBLIC_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE5Ch5+mGfndJIney2+6g7+lbsNMeM
O9gYi13U3ipzRa13jlButZX+ww32GV5tJnL9RqH+RMQN4UZA0qStZNScoQ==
-----END PUBLIC KEY-----"#;

const DEFAULT_TURMS_DISCOVERY_URL: &str = "wss://discovery.gravitalia.com";
const DEFAULT_STUN_SERVER: &str = "stun:stun.l.google.com:19302";
const AUTH_TURN_SERVER: &str = "turn:turn.gravitalia.com:3478";

/// Init account.
pub async fn init(
    state: &Arc<Mutex<State>>,
    token: Option<String>,
    turms_url: Option<String>,
) -> Result<(), String> {
    let token = token.filter(|s| !s.is_empty());

    // RTC server configuration.
    let default_rtc = RTCIceServer {
        urls: vec![DEFAULT_STUN_SERVER.to_string()],
        ..Default::default()
    };

    let config = match token.clone() {
        Some(username) => Config {
            turms_url: Some(
                turms_url.unwrap_or(DEFAULT_TURMS_DISCOVERY_URL.to_string()),
            ),
            rtc: vec![
                RTCIceServer {
                    urls: vec![AUTH_TURN_SERVER.to_string()],
                    username,
                    ..Default::default()
                },
                default_rtc,
            ],
        },
        None => Config {
            turms_url,
            rtc: vec![default_rtc],
        },
    };

    let mut locked_state = state.lock().await;

    // libturms configuration.
    let yamlconfig =
        serde_yaml::to_string(&config).map_err(|e| e.to_string())?;
    let (mut turms, _receiver) =
        Turms::from_config(ConfigFinder::<String>::Text(yamlconfig))
            .map_err(|e| e.to_string())?;

    if let Some(ref token) = token {
        turms = turms.connect_ws(token).await.unwrap();
    }

    locked_state.turms = Some(turms);

    // Generate user.
    let mut user = match token {
        Some(ref token) => {
            let user = state
                .lock()
                .await
                .token
                .decode(token)
                .map_err(|e| e.to_string())?;
            User::new(&user.subject, &user.subject)
        },
        None => User::new(&GUEST.to_lowercase(), &GUEST.to_string()),
    };

    let olm_account = libturms::p2p::save_account()
        .await
        .map_err(|_| "security not initialized".to_string())?;

    user = user.with_config(config).with_account(olm_account);

    locked_state
        .database
        .create_user(&user)
        .map_err(|_| "user not created".to_string())?;
    locked_state.user = Some(user);

    Ok(())
}

/// Deeplink handler.
pub fn handler(state: Arc<Mutex<State>>, urls: Vec<tauri::Url>) {
    let token = urls.first().and_then(|url| {
        let parser_url = url::Url::parse(url.as_str()).ok()?;
        parser_url
            .query_pairs()
            .find(|(key, _)| key == "token")
            .map(|(_, value)| value.into_owned())
    });

    if let Some(token) = token {
        tauri::async_runtime::spawn(async move {
            if let Err(error) = init(&state, Some(token), None).await {
                log::error!(
                    "failed to initialize user via deep link: {error:?}",
                );
            }
        });
    } else {
        tauri::async_runtime::spawn(async move {
            if let Err(error) = init(&state, None, None).await {
                log::error!("failed to initialize guest user: {error:?}");
            }
        });
    }
}
