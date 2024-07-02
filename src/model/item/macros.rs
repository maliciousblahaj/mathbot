#[macro_export]
macro_rules! item_query_by_key_value {
    ($keyname:expr, $keyvalue:expr) => {
        sqlx::query_as!(
            Item,
            "SELECT id, name_id, emoji_id, image_url, display_name, item_type, price, description, 
            multiplier, mps FROM Items WHERE ?=?", $keyname, $keyvalue
        )
    };
}

#[macro_export]
macro_rules! item_query_by_key_value_alias {
    ($keyname:expr, $keyvalue:expr) => {
        sqlx::query_as!(
            Item,
            "SELECT id, name_id, emoji_id, image_url, display_name, item_type, price, description, 
            multiplier, mps FROM Items WHERE ? LIKE ?", $keyname, $keyvalue
        )
    };
}

//AHHH WHY DO I HAVE TO WRITE A MACRO TO WRITE A MACRO TO WRITE A MACRO????
#[macro_export]
macro_rules! item_query_by_key {
    ($key:expr, $database:expr) => {
        match $key {
            ItemQueryKey::id(id) => 
            {
                let (key, id) = ($key.as_ref(), &id);
                item_query_by_key_value!(key, id)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchItem(e))
            },
            ItemQueryKey::name_id(name_id) => 
            {
                let (key, name_id) = ($key.as_ref(), &name_id);
                item_query_by_key_value!(key, name_id)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchItem(e))
            },
            ItemQueryKey::name_id_incomplete(name_id_incomplete) =>             
            {
                let (key, name_id_incomplete) = ($key.as_ref(), &name_id_incomplete);
                item_query_by_key_value!(key, name_id_incomplete)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchItem(e))
            },
        }
    };
}