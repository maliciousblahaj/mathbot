use std::sync::Arc;

use tokio::sync::Mutex;
use crate::{Error, Result};
use super::ModelController;

#[allow(unused)]
pub struct AccountController {
    mc: Arc<Mutex<ModelController>>,
    id: i64,
}

impl AccountController {
    pub fn new(mc: &Arc<Mutex<ModelController>>, id: i64) -> Self {
        Self {
            mc: mc.clone(),
            id,
        }
    }

    pub fn get_id(&self) -> &i64 {
        &self.id
    }

    pub async fn fetch_account(&self) -> Result<Account> {
        let id = self.get_id();
        let query = sqlx::query!(
            "SELECT * FROM Accounts WHERE id=?", id
        );

        let account = query
            .fetch_one(&self.mc.lock().await.database)
            .await
            .map_err(|e| Error::FailedToFetchAccount(e))?;

        Ok(
        Account {
            id: account.id,
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


pub struct Account {
    pub id: i64,
    pub user_id: i64,
    pub created: i64,
    pub balance: f64,
    pub smps_solved: i64,
    pub is_banned: bool,
    pub mine_slots: i64,
    pub previous_claim: i64,
    pub awaiting_claim: i64,
    pub user_name: String,
    pub user_bio: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_url: String,
    pub next_username_update: i64,
    pub is_admin: bool,
}