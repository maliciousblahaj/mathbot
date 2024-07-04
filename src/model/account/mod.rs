use std::sync::Arc;

use tokio::sync::Mutex;
use crate::{account_query_by_key, model::ModelController, Error, Result};

#[macro_use]
pub mod macros;

pub struct AccountController {
    mc: Arc<Mutex<ModelController>>,
    key: AccountQueryKey,
    fetched_account: Option<Account>,
}

//WHEN SIGNING UP, PREVENT USERS FROM USING JUST NUMBERS AS THEIR USERNAME
impl AccountController{
    pub fn new(mc: &Arc<Mutex<ModelController>>, key: AccountQueryKey) -> Self {
        Self {
            mc: mc.clone(),
            key: key,
            fetched_account: None,
        }
    }

    pub async fn fetch_account(&mut self) -> Result<Account> {
        let fetched_account = account_query_by_key!(&self.key, &self.mc.lock().await.database)?;
        self.fetched_account = Some(fetched_account.clone());
        self.key = AccountQueryKey::id(fetched_account.id.clone());
        Ok(fetched_account)
    }

    pub async fn fetch_slots(&self) -> Result<Vec<Slot>> {
        let AccountQueryKey::id(id) = &self.key else {return Err(Error::FetchedSlotsBeforeFetchingAccount);};
        sqlx::query_as!(
            Slot,
            "SELECT id, account_id, item_id FROM Slots WHERE id =
            (SELECT id FROM Accounts WHERE account_id=?)", id
        )
            .fetch_all(&self.mc.lock().await.database)
            .await
            .map_err(|e| Error::FailedToFetchAccountSlots(e))
    }
}

pub struct Slot {
    pub id: i64,
    pub account_id: i64,
    pub item_id: i64
}


#[allow(non_camel_case_types)]
#[derive(strum_macros::AsRefStr, Clone)]
pub enum AccountQueryKey{
    id(i64),
    user_id(i64),
    username(String),
    username_incomplete(String),
}

#[derive(Clone)]
pub struct Account {
    pub id: i64,
    pub user_id: i64,
    pub created: i64, //-62167222408 for year 0
    pub balance: f64,
    pub smps_solved: i64,
    pub is_banned: i64,
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
    pub fn is_banned(&self) -> bool {
        if self.is_banned == 0 {false}
            else {true}
    }

    pub fn is_admin(&self) -> bool {
        if self.is_admin == 0 {false}
            else {true}
    }
}