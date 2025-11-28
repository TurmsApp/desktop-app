use rand::rand_core;
use serde::Serialize;
use tauri_plugin_log::log;
use thiserror::Error;

/// Custom error type.
pub type Result<T> = std::result::Result<T, TurmsError>;

#[derive(Error, Debug)]
pub enum TurmsError {
    #[error(transparent)]
    Turms(#[from] libturms::error::Error),

    #[error("sqlite error")]
    Database(#[from] rusqlite::Error),

    #[error(transparent)]
    Security(#[from] rand_core::OsError),
    #[error(transparent)]
    Key(#[from] keyring::Error),

    #[error(transparent)]
    JsonSerialization(#[from] serde_json::Error),
    #[error(transparent)]
    YamlSerialization(#[from] serde_yaml::Error),

    #[error("mutex is poisoned")]
    PoisonedMutex,
    #[error("user does not exist")]
    UserNotExists,
    #[error("turms instance is not initialized")]
    TurmsInstanceNotInitialized,
    #[error("invalid session type")]
    InvalidSession,

    #[error("missing {0} entry on database")]
    MissingEntry(String),
}

impl Serialize for TurmsError {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        log::error!("invokation emitted: {:?}", self);
        serializer.serialize_str(&self.to_string())
    }
}
