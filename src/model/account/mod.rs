use std::sync::Arc;

use tokio::sync::Mutex;
use crate::{account_query_by_key, model::ModelController, Error, Result};

#[macro_use]
pub mod macros;

#[allow(unused)]
pub struct AccountController {
    mc: Arc<Mutex<ModelController>>,
    key: AccountQueryKey,
}

//WHEN SIGNING UP, PREVENT USERS FROM USING JUST NUMBERS AS THEIR USERNAME
impl AccountController{
    pub fn new(mc: &Arc<Mutex<ModelController>>, key: AccountQueryKey) -> Self {
        Self {
            mc: mc.clone(),
            key: key
        }
    }

    pub async fn fetch_account(&self) -> Result<Account> {
        let fetch_result = account_query_by_key!(&self.key, &self.mc.lock().await.database);
        fetch_result
    }
}

#[allow(non_camel_case_types)]
#[derive(strum_macros::AsRefStr, Clone)]
pub enum AccountQueryKey{
    id(i64),
    user_id(i64),
    user_name(String),
    user_name_incomplete(String),
}

pub struct Account {
    pub id: i64,
    pub user_id: i64,
    pub created: i64,
    pub balance: f64,
    pub smps_solved: i64,
    pub is_banned: i64,
    pub mine_slots: i64,
    pub previous_claim: i64,
    pub awaiting_claim: i64,
    pub user_name: String,
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