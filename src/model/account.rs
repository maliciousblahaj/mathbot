use std::sync::Arc;

use tokio::sync::Mutex;
use crate::Result;
use super::ModelController;

pub struct Account {
    mc: Arc<Mutex<ModelController>>,
    id: u64,
    userid: u64,
}

impl Account {
    pub fn from_userid(mc: &Arc<Mutex<ModelController>>, userid: u64) -> Result<Self> {
        let id = 0; //replace with getting it from the database
        Ok(
            Self {
                mc: mc.clone(),
                id,
                userid,
            }
        )
    }

    pub fn from_id(mc: &Arc<Mutex<ModelController>>, id: u64) -> Result<Self> {
        let userid = 0; //replace with getting it from the database
        Ok(
            Self {
                mc: mc.clone(),
                id,
                userid,
            }
        )
    }
}