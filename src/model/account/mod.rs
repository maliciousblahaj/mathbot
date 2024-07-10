use std::sync::Arc;

use crate::{account_query_by_key, get_current_timestamp_secs_i64, model::ModelController, Error, Result};

#[macro_use]
pub mod macros;

pub struct AccountController {
    mc: Arc<ModelController>,
    key: AccountQueryKey,
    fetched_account: Option<Account>,
}

//WHEN SIGNING UP, PREVENT USERS FROM USING JUST NUMBERS AS THEIR USERNAME
impl AccountController{
    pub fn new(mc: &Arc<ModelController>, key: AccountQueryKey) -> Self {
        Self {
            mc: mc.clone(),
            key: key,
            fetched_account: None,
        }
    }

    pub async fn fetch_account(&mut self) -> Result<Account> {
        let fetched_account = account_query_by_key!(&self.key, &self.mc.database)?;
        self.fetched_account = Some(fetched_account.clone());
        self.key = AccountQueryKey::id(fetched_account.id.clone());
        Ok(fetched_account)
    }

    pub async fn fetch_slots(&self) -> Result<Vec<Slot>> {
        let AccountQueryKey::id(id) = &self.key else {return Err(Error::FetchedSlotsBeforeFetchingAccount);};
        sqlx::query_as!(
            Slot,
            "
            SELECT 
                Slots.id, 
                Slots.account_id, 
                Slots.item_id,
                Items.emoji_id,
                Items.display_name,
                Items.mps
            FROM Slots 
                INNER JOIN Items ON Items.id = Slots.item_id
            WHERE account_id = ?
            ", id
        )
            .fetch_all(&self.mc.database)
            .await
            .map_err(|e| Error::FailedToFetchAccountSlots(e))
    }

    pub async fn fetch_inventory(&self) -> Result<Vec<InventoryItem>> {
        let AccountQueryKey::id(id) = &self.key else {return Err(Error::FetchedInventoryBeforeFetchingAccount);};
        sqlx::query_as!(
            InventoryItem,
            "
            SELECT 
                Inventory.account_id, 
                Items.id as item_id,
                Inventory.count,
                Items.emoji_id,
                Items.display_name
            FROM Inventory
                INNER JOIN Items ON Items.id = Inventory.item_id
            WHERE account_id=?
            AND Inventory.count > 0
            ", id
        )
            .fetch_all(&self.mc.database)
            .await
            .map_err(|e| Error::FailedToFetchAccountInventory(e))
    }

    pub async fn delete_account(&mut self) -> Result<()> {
        let AccountQueryKey::id(id) = &self.key else {return Err(Error::DeletedAccountBeforeFetchingAccount);};
        sqlx::query!(
            "DELETE FROM Accounts WHERE id=?", id
        )
            .execute(&self.mc.database)
            .await
            .map_err(|e| Error::FailedToDeleteAccount(e))?;
        Ok(())
    }
}

pub struct Slot {
    pub id: i64,
    pub account_id: i64,
    pub item_id: i64,
    pub emoji_id: Option<String>,
    pub display_name: String,
    pub mps: Option<f64>,
}

pub struct InventoryItem {
    pub account_id: i64,
    pub item_id: i64,
    pub count: i64,
    pub emoji_id: Option<String>,
    pub display_name: String,
}


#[allow(non_camel_case_types)]
#[derive(strum_macros::AsRefStr, Clone, Debug)]
pub enum AccountQueryKey{
    id(i64),
    user_id(i64),
    username(String),
    username_incomplete(String),
}

#[derive(Clone, Debug)]
pub struct Account {
    pub id: i64,
    pub user_id: i64,
    pub created: i64, //-62167222408 for year 0, //561769200 for 1987
    pub balance: f64,
    pub smps_solved: i64,
    pub banned: i64,
    pub mine_slots: i64,
    pub previous_claim: i64,
    pub awaiting_claim: i64,
    pub username: String,
    pub user_bio: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_url: String,
    pub next_username_update: i64,
    pub is_admin: i64,
}

impl Account {
    pub fn is_banned(&self) -> Result<bool> {
        if self.banned < get_current_timestamp_secs_i64()? {Ok(false)}
            else {Ok(true)}
    }

    pub fn is_admin(&self) -> bool {
        if self.is_admin == 0 {false}
            else {true}
    }
}