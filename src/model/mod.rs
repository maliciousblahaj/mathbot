pub mod item;
pub mod account;

use crate::{get_current_timestamp_secs, Error, Result};

pub struct ModelController {
    database: sqlx::SqlitePool,
}

impl ModelController {
    pub fn new(database: sqlx::SqlitePool) -> Self {
        Self {
            database,
        }
    }

    pub async fn create_account(&self, user_id: i64, username: String, avatar_url: String) -> Result<()> {
        let timestamp = get_current_timestamp_secs()? as i64;
        sqlx::query!("INSERT INTO Accounts (user_id, created, username, avatar_url) VALUES (?,?,?,?)", user_id, timestamp, username, avatar_url)
            .execute(&self.database)
            .await
            .map_err(|e| Error::FailedToCreateAccount(e))?;

        Ok(())
    }

    pub fn get_database(&self) -> &sqlx::SqlitePool {
        &self.database
    }
}
