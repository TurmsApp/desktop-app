//! SQLite manager with pre-made queries.

use anyhow::{Result, anyhow};
use chrono::{TimeZone, Utc};
use libturms::Config;
use rusqlite::Connection;
use rusqlite::ToSql;

use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use crate::models::user::User;

/// Methods to identify one or more rows.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Get {
    Id(String),
    Token(String),
    Me,
}

/// SQLite manager.
#[derive(Debug, Clone)]
pub struct Database {
    connection: Arc<Mutex<Connection>>,
}

impl Database {
    /// Create a new [`Database`] instance.
    pub fn new<P: AsRef<Path>>(path: P, key: String) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.pragma_update(None, "key", format!("aes256:{key}"))?;
        conn.pragma_update(None, "foreign_keys", "ON")?;

        Ok(Database {
            connection: Arc::new(Mutex::new(conn)),
        })
    }

    /// Insert a user in database.
    pub fn create_user(&self, user: &User) -> Result<()> {
        let config = user
            .config
            .clone()
            .map(|c| {
                serde_json::to_string(&c)
                    .map_err(|err| anyhow!("Failed to serialize config: {err}"))
            })
            .transpose()?
            .unwrap_or_default();

        let account = user.account.clone().unwrap_or_default();

        self.connection
            .lock()
            .map_err(|_| anyhow!("mutex is poisoned"))?
            .execute(
                "INSERT INTO users
                    (id, trust_level, username, relation_date, config, account)
                    VALUES (?, 0, ?, ?, ?, ?)",
                [
                    &user.id as &dyn ToSql,
                    &user.username as &dyn ToSql,
                    &Utc::now().timestamp() as &dyn ToSql,
                    &config as &dyn ToSql,
                    &account as &dyn ToSql,
                ],
            )?;

        Ok(())
    }

    /// Get a user from database.
    pub fn get_user(&self, identifier: Get) -> Result<User> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| anyhow!("mutex is poisoned"))?;

        match identifier {
            Get::Id(id) => Ok(conn
                .query_row(
                    "SELECT
                        username, trust_level, avatar, relation_date, public_key, state, account
                        FROM users WHERE id = ?",
                    [id.clone()],
                    move |row| {
                        Ok(User {
                            id,
                            username: row.get(0)?,
                            trust_level: row.get(1)?,
                            avatar: row.get(2)?,
                            relation: Utc
                                .timestamp_opt(row.get::<usize, i64>(3)?, 0)
                                .earliest()
                                .unwrap_or(Utc::now()),
                            public_key: row.get(4)?,
                            account: None,
                            ..Default::default()
                        })
                    },
                )?),
            Get::Token(_jwt) => {
                // Read JWT token and extract ID, then fetch.
                //self.get_user(id)?
                unimplemented!();
            },
            Get::Me => {
                Ok(conn
                .query_row(
                    "SELECT
                        id, username, avatar, relation_date, config, account
                        FROM users
                        WHERE config IS NOT NULL AND config <> ''",
                    [],
                    move |row| {
                        let config = serde_json::from_str::<Config>(&row.get::<usize, String>(4)?).unwrap();
                        Ok(User {
                            id: row.get(0)?,
                            trust_level: 1,
                            username: row.get(1)?,
                            avatar: row.get(2)?,
                            relation: Utc
                                .timestamp_opt(row.get::<usize, i64>(3)?, 0)
                                .earliest()
                                .unwrap_or(Utc::now()),
                            config: Some(config),
                            account: Some(row.get::<usize, String>(5)?).filter(|s| !s.is_empty()),
                            ..Default::default()
                        })
                    },
                )?)
            }
        }
    }

    pub fn get_conversations(&self) -> Result<Vec<User>> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| anyhow!("mutex is poisoned"))?;

        let mut statement = conn
            .prepare("SELECT
                        id, trust_level, username, avatar, relation_date, public_key, state
                        FROM users
                        WHERE config IS NULL OR config = ''")?;

        let users = statement
            .query_map([], |row| {
                Ok(User {
                    id: row.get(0)?,
                    trust_level: row.get(1)?,
                    username: row.get(2)?,
                    avatar: row.get(3)?,
                    relation: Utc
                        .timestamp_opt(row.get::<usize, i64>(4)?, 0)
                        .earliest()
                        .unwrap_or(Utc::now()),
                    //public_key: row.get(5)?,
                    ..Default::default()
                })
            })?
            .filter_map(|user| user.ok());

        Ok(users.collect())
    }

    /// Create tables if not exists.
    pub fn create_tables(&self) -> Result<()> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| anyhow!("mutex is poisoned"))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id              TEXT PRIMARY KEY,
                trust_level     INTEGER NOT NULL,
                username        TEXT NOT NULL,
                avatar          TEXT,
                relation_date   INTEGER NOT NULL,
                config          TEXT,
                account         TEXT,
                public_key      BLOB,
                state           BLOB)",
            (),
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
            id          TEXT PRIMARY KEY,
            user_id     TEXT NOT NULL,
            ciphertext  TEXT,
            flags       INTEGER,
            reference   REFERENCES messages(id),
            state       BLOB,
            FOREIGN KEY (user_id) REFERENCES users(id))",
            (),
        )?;

        Ok(())
    }
}
