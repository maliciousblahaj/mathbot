use std::{str::FromStr, sync::Arc};
use crate::{Error, Result};
use super::ModelController;

#[macro_use]
pub mod macros;

pub struct ItemController {
    mc: Arc<ModelController>,
    key: ItemQueryKey,
}

impl ItemController {
    pub fn new(mc: &Arc<ModelController>, key: ItemQueryKey) -> Self {
        Self {
            mc: mc.clone(),
            key: key
        }
    }

    pub async fn fetch_item(&self) -> Result<Item> {
        let fetch_result = item_query_by_key!(&self.key, &self.mc.database);
        fetch_result
    }
}

#[derive(strum_macros::AsRefStr, strum_macros::EnumString)]
pub enum ItemType {
    Token,
    GraphicsCard,
    AntiVirus,
    TestItem,
}

impl ItemType {
    pub fn get_string(&self) -> &str {
        match self {
            Self::Token => "Token",
            Self::GraphicsCard => "Graphics card",
            Self::AntiVirus => "Antivirus",
            Self::TestItem => "Test item",
        }
    }
}

//PANICS!! Fix this
impl Into<ItemType> for String {
    fn into(self) -> ItemType {
        ItemType::from_str(self.as_str()).unwrap()
    }
}


#[allow(non_camel_case_types)]
#[derive(strum_macros::AsRefStr, Clone)]
pub enum ItemQueryKey {
    id(i64),
    name_id(String),
    name_id_incomplete(String),
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
    pub fn is_for_sale(&self) -> bool {
        self.price.is_some()
    }
}