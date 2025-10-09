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

        Ok(Database {
            connection: Arc::new(Mutex::new(conn)),
        })
    }

    /// Insert a user in database.
    pub fn create_user(&self, user: &User, config: Option<Config>) -> Result<()> {
        let config = config
            .map(|c| {
                serde_json::to_string(&c)
                    .map_err(|err| anyhow!("Failed to serialize config: {err}"))
            })
            .transpose()?
            .unwrap_or_default();

        self.connection
            .lock()
            .map_err(|_| anyhow!("mutex is poisoned"))?
            .execute(
                "INSERT INTO users (id, username, relation_date, config) VALUES (?, ?, ?, ?)",
                [
                    &user.id as &dyn ToSql,
                    &user.username as &dyn ToSql,
                    &Utc::now().timestamp() as &dyn ToSql,
                    &config as &dyn ToSql,
                ],
            )?;

        Ok(())
    }

    /// Get a user from database.
    pub fn get_user(&self, identifier: Get) -> Result<(User, Option<Config>)> {
        let conn = self
            .connection
            .lock()
            .map_err(|_| anyhow!("mutex is poisoned"))?;

        match identifier {
            Get::Id(id) => Ok(conn
                .query_row(
                    "SELECT username, avatar, relation_date, long_key FROM users WHERE id = ?",
                    [id.clone()],
                    move |row| {
                        Ok((User {
                            id,
                            username: row.get(0)?,
                            avatar: row.get(1)?,
                            token: None,
                            relation: Utc
                                .timestamp_opt(row.get::<usize, i64>(2)?, 0)
                                .earliest()
                                .unwrap_or(Utc::now()),
                        }, None))
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
                    "SELECT id, username, avatar, relation_date, public_key, private_key, config FROM users WHERE config IS NOT NULL AND config <> '';",
                    [],
                    move |row| {
                        Ok((User {
                            id: row.get(0)?,
                            username: row.get(1)?,
                            avatar: row.get(2)?,
                            token: None,
                            relation: Utc
                                .timestamp_opt(row.get::<usize, i64>(3)?, 0)
                                .earliest()
                                .unwrap_or(Utc::now()),
                        }, Some(serde_json::from_str(&row.get::<usize, String>(6)?).unwrap())))
                    },
                )?)
            }
        }
    }

    /// Create tables if not exists.
    pub fn create_tables(&self) -> Result<()> {
        self.connection
            .lock()
            .map_err(|_| anyhow!("mutex is poisoned"))?
            .execute(
                "CREATE TABLE IF NOT EXISTS users (
                id              TEXT PRIMARY KEY,
                username        TEXT NOT NULL,
                avatar          TEXT,
                relation_date   INTEGER NOT NULL,
                config          TEXT,
                public_key      BLOB,
                private_key     BLOB)",
                (),
            )?;
        Ok(())
    }
}
