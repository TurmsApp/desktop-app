//! SQLite manager with pre-made queries.

use chrono::{TimeZone, Utc};
use rusqlite::{Connection, Result};

use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use crate::models::user::User;

/// Methods to identify one or more rows.
pub enum Get {
    Id(String),
    Token(String),
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

    pub fn set_user(&self, _user: User) -> Result<()> {
        Ok(())
    }

    /// Get a user from database.
    pub fn get_user(&self, identifier: Get) -> Result<User> {
        match identifier {
            Get::Id(id) => self.connection.lock().unwrap().query_row(
                "SELECT username, avatar, relation WHERE id = ?",
                [id.to_owned()],
                move |row| {
                    Ok(User {
                        id,
                        username: row.get(0)?,
                        avatar: row.get(1)?,
                        token: None,
                        relation: Utc
                            .timestamp_opt(row.get::<usize, i64>(2)?, 0)
                            .earliest()
                            .unwrap_or(Utc::now()),
                    })
                },
            ),
            Get::Token(_jwt) => {
                // Read JWT token and extract ID, then fetch.
                //self.get_user(id)?
                unimplemented!();
            }
        }
    }

    /// Create tables if not exists.
    pub fn create_tables(&self) -> Result<()> {
        self.connection.lock().unwrap().execute(
            "CREATE TABLE IF NOT EXISTS users (
                id       INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL,
                avatar   TEXT,
                relation INTEGER NOT NULL,
            )",
            (),
        )?;

        Ok(())
    }
}
