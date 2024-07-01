use std::str::FromStr;
use crate::Result;
use account::Account;
use item::{Item, ItemType};

use crate::Error;

pub mod item;
pub mod account;

pub struct ModelController {
    database: sqlx::SqlitePool,
}


impl ModelController {
    pub fn new(database: sqlx::SqlitePool) -> Self {
        Self {
            database,
        }
    }

    pub async fn get_item_from_alias(&self, alias: &String) -> Result<Item> {
        let query = sqlx::query!(
            "SELECT * FROM Items WHERE name_id LIKE ?", alias
        );

        let item = query
            .fetch_one(&self.database)
            .await
            .map_err(|e| Error::FailedToFetchItem(e))?;

        Ok(
        Item {
            id: item.id,
            name_id: item.name_id,
            emoji_id: item.emoji_id,
            image_url: item.image_url,
            display_name: item.display_name,
            item_type: ItemType::from_str(&item.item_type).map_err(|e| Error::FailedToParseItemType(e))?,
            price: item.price,
            description: item.description,
            multiplier: item.multiplier,
            mps: item.mps,
        })
    }
    
    pub async fn get_account_from_user_id(&self, user_id: i64) -> Result<Account> {
        let query = sqlx::query!(
            "SELECT * FROM Accounts WHERE user_id=?", user_id
        );

        let account = query
            .fetch_one(&self.database)
            .await
            .map_err(|e| Error::FailedToFetchAccount(e))?;

        Ok(
        Account {
            id: account.id.ok_or(Error::DatabaseFailedToGetAccountId)?,
            user_id: account.user_id,
            created: account.created,
            balance: account.balance,
            smps_solved: account.smps_solved,
            is_banned: if account.is_banned == 0 {false} else {true},
            mine_slots: account.mine_slots,
            previous_claim: account.previous_claim,
            awaiting_claim: account.awaiting_claim,
            user_name: account.user_name,
            user_bio: account.user_bio,
            pronouns: account.pronouns,
            avatar_url: account.avatar_url,
            next_username_update: account.next_username_update,
            is_admin: if account.is_admin == 0 {false} else {true}
        })
    }

}