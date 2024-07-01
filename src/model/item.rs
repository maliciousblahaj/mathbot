use std::{str::FromStr, sync::Arc};
use crate::{Error, Result};

use tokio::sync::Mutex;

use super::ModelController;

pub struct ItemController {
    mc: Arc<Mutex<ModelController>>,
    id: i64,
}

impl ItemController {
    pub fn new(mc: &Arc<Mutex<ModelController>>, id: i64) -> Self {
        Self {
            mc: mc.clone(),
            id,
        }
    }

    pub fn get_id(&self) -> &i64 {
        &self.id
    }

    pub async fn fetch_item(&self) -> Result<Item> {
        let id = self.get_id();
        let query = sqlx::query!(
            "SELECT * FROM Items WHERE id=?", id
        );

        let item = query
            .fetch_one(&self.mc.lock().await.database)
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
}

#[derive(strum_macros::AsRefStr, strum_macros::EnumString)]
pub enum ItemType {
    Token,
    GraphicsCard,
    AntiVirus,
    TestItem,
}


pub struct Item {
    pub id: i64,
    pub name_id: String,
    pub emoji_id: Option<String>,
    pub image_url: Option<String>,
    pub display_name: String,
    pub item_type: ItemType,
    pub price: Option<i64>,
    pub description: Option<String>,
    pub multiplier: Option<f64>,
    pub mps: Option<f64>,
}

impl Item {
    /*
    pub fn new(id: i64, name_id: String, emoji_id: Option<String>, image_url: Option<String>, display_name: String, item_type: ItemType, price: Option<i64>, description: Option<String>, multiplier: Option<f64>, mps: Option<f64>) -> Self {
        Self{id, name_id, emoji_id, image_url, display_name, item_type, price, description, multiplier, mps}
    }*/

    pub fn is_for_sale(&self) -> bool {
        self.price.is_some()
    }
}