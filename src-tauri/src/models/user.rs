//! User model.

use super::keys::Keys;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
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
    /// Public and private (if user is sender) keys.
    #[serde(skip_serializing, skip_deserializing)]
    pub keys: Keys,
}
