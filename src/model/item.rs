use std::sync::Arc;
use crate::Result;

use tokio::sync::Mutex;

use super::ModelController;

pub struct Item {
    mc: Arc<Mutex<ModelController>>,
    id: u64,
    nameid: u64,
}

impl Item {
    pub fn new(mc: &Arc<Mutex<ModelController>>, id: u64) -> Result<Self> {
        let nameid = 0; //replace with getting it from the database
        Ok(
            Self {
                mc: mc.clone(),
                id,
                nameid,
            }
        )
    }
}