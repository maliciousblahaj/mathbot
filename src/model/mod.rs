pub mod item;
pub mod account;

use item::Item;
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
        sqlx::query!("INSERT INTO Accounts (user_id, created, balance, username, avatar_url) VALUES (?,?,5000.0,?,?)", user_id, timestamp, username, avatar_url)
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
            .map_err(|e: sqlx::Error| Error::FailedToCheckIfAccountExists(e))?;
        if exists.count > 0 {
            return Ok(true)
        }
        Ok(false)
    }

    pub fn get_database(&self) -> &sqlx::SqlitePool {
        &self.database
    }

    pub async fn get_shop(&self) -> Result<Vec<ShopItem>> {
        sqlx::query_as!(ShopItem, "SELECT name_id, emoji_id, display_name, price, mps FROM Items WHERE price IS NOT NULL")
            .fetch_all(&self.database)
            .await
            .map_err(|e: sqlx::Error| Error::FailedToFetchShop(e))
    }
}

pub struct ShopItem {
    pub name_id: String,
    pub emoji_id: Option<String>,
    pub price: Option<i64>,
    pub display_name: String,
    pub mps: Option<f64>,
}

impl ShopItem {
    pub fn from_item(item: Item) -> Self {
        Self {
            name_id: item.name_id,
            emoji_id: item.emoji_id,
            price: item.price,
            display_name: item.display_name,
            mps: item.mps
        }
    }
}