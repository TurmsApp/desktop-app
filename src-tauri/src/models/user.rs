//! User model.

use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use libturms::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
/// Represents receiver and sender.
pub struct User {
    /// Unique identifier in Turms format.
    pub id: String,
    pub trust_level: u8,
    /// Public name.
    pub username: String,
    /// Public profile picture.
    pub avatar: Option<String>,
    /// Date on which the two users first communicated.
    #[serde(with = "ts_milliseconds")]
    pub relation: DateTime<Utc>,
    /// User session.
    pub session: Option<String>,
    /// Custom configuration for Turms.
    pub config: Option<Config>,
    /// Serialized OLM account for key pair.
    pub account: Option<String>,
}

impl User {
    /// Create a new [`User`] instance.
    pub fn new<T: ToString>(id: T, username: T) -> Self {
        Self {
            id: id.to_string(),
            username: username.to_string(),
            ..Default::default()
        }
    }

    /// Set `config` field.
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    /// Set `account` field.
    pub fn with_account(mut self, account: String) -> Self {
        self.account = Some(account);
        self
    }

    /// Set `session` field.
    pub fn with_session(mut self, session: Option<String>) -> Self {
        self.session = session;
        self
    }
}
