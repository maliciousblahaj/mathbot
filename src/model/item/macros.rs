//BOOOOOOOOOOOOOOOOOOOOOILERPLATE

#[macro_export]
macro_rules! item_query_by_key_id {
    ($keyvalue:expr) => {
        sqlx::query_as!(
            Item,
            "SELECT id, name_id, emoji_id, image_url, display_name, item_type, price, description, 
            multiplier, mps FROM Items WHERE id=?", $keyvalue
        )
    };
}

#[macro_export]
macro_rules! item_query_by_key_name_id {
    ($keyvalue:expr) => {
        sqlx::query_as!(
            Item,
            "SELECT id, name_id, emoji_id, image_url, display_name, item_type, price, description, 
            multiplier, mps FROM Items WHERE name_id=? COLLATE NOCASE", $keyvalue
        )
    };
}

#[macro_export]
macro_rules! item_query_by_key_name_id_incomplete {
    ($keyvalue:expr) => {
        sqlx::query_as!(
            Item,
            "SELECT id, name_id, emoji_id, image_url, display_name, item_type, price, description, 
            multiplier, mps FROM Items WHERE name_id LIKE ? COLLATE NOCASE", $keyvalue
        )
    };
}

#[macro_export]
macro_rules! item_query_by_key {
    ($key:expr, $database:expr) => {
        match $key {
            ItemQueryKey::id(id) => 
            {
                item_query_by_key_id!(id)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchItem(e))
            },
            ItemQueryKey::name_id(name_id) => 
            {
                item_query_by_key_name_id!(name_id)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchItem(e))
            },
            ItemQueryKey::name_id_incomplete(name_id_incomplete) =>             
            {
                item_query_by_key_name_id_incomplete!(name_id_incomplete)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchItem(e))
            },
        }
    };
}