pub mod item;
pub mod account;

use uuid::Uuid;

use crate::{get_current_timestamp_secs_i64, Error, Result};

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
        let mut username = username;
        if self.username_exists(&username).await? {
            username = Uuid::new_v4().to_string();
        }
        let timestamp = get_current_timestamp_secs_i64()?;
        sqlx::query!("INSERT INTO Accounts (user_id, created, username, avatar_url) VALUES (?,?,?,?)", user_id, timestamp, username, avatar_url)
            .execute(&self.database)
            .await
            .map_err(|e| Error::FailedToCreateAccount(e))?;

        Ok(())
    }

    ///This function returns true if the username exists
    pub async fn username_exists(&self, username: &String) -> Result<bool> {
        let exists = sqlx::query!("SELECT COUNT(1) as count FROM Accounts WHERE username=?", username)
            .fetch_one(&self.database)
            .await
            .map_err(|e| Error::FailedToCheckIfAccountExists(e))?;
        if exists.count > 0 {
            return Ok(true)
        }
        Ok(false)
    }

    pub fn get_database(&self) -> &sqlx::SqlitePool {
        &self.database
    }
}
