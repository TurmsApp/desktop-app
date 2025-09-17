//! SQLite manager with pre-made queries.

use crate::models::{keys::Keys, user::User};
use chrono::{TimeZone, Utc};
use rusqlite::{Connection, Result};
use std::path::Path;

/// Methods to identify one or more rows.
pub enum Get {
    Id(String),
    Token(String),
}

/// Easly manage SQLite database.
///
/// For future, if database changes, it would be easier to
/// switch database.
pub struct Database {
    connection: Connection,
    /// Actual user.
    pub me: Option<User>,
}

impl Database {
    /// Create a new [`Database`] instance.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;

        //conn.query_row("SELECT id, username WHERE id = ?", [], |row| row.get(0));

        Ok(Database {
            connection: conn,
            me: None,
        })
    }

    /// Get a user from database.
    pub fn get_user(&self, identifier: Get) -> Result<User> {
        match identifier {
            Get::Id(id) => {
                self.connection.query_row(
                    "SELECT username, avatar, relation WHERE id = ?",
                    [id.to_owned()],
                    move |row| {
                        Ok(User {
                            id,
                            username: row.get(0)?,
                            avatar: row.get(1)?,
                            token: None,
                            keys: Keys {
                                public_key: None,
                                private_key: None,
                                ratchet: None,
                            },
                            relation: Utc
                                .timestamp_opt(
                                    row.get::<usize, i64>(2)? as i64,
                                    0,
                                )
                                .earliest()
                                .unwrap_or(Utc::now()),
                        })
                    },
                )
            },
            Get::Token(jwt) => {
                // Read JWT token and extract ID, then fetch.
                //self.get_user(id)?
                Ok(User {
                    id: String::default(),
                    username: String::default(),
                    avatar: None,
                    token: Some(jwt),
                    keys: Keys::default(),
                    relation: Utc::now(),
                })
            },
        }
    }

    /// Create tables if not exists.
    pub fn create_tables(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE users (
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
