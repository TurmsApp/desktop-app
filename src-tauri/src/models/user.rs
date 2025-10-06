//! User model.

use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
/// Represents receiver and sender.
pub struct User {
    /// Unique identifier in Turms format.
    pub id: String,
    /// Public name.
    pub username: String,
    /// Public profile picture.
    pub avatar: Option<String>,
    /// Date on which the two users first communicated.
    #[serde(with = "ts_milliseconds")]
    pub relation: DateTime<Utc>,
    /// If user is sender.
    /// JsonWebToken (JWT) plain string.
    pub token: Option<String>,
    /* /// Public and private (if user is sender) keys.
    #[serde(skip_serializing, skip_deserializing)]
    pub keys: Keys, */
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
}
