#[macro_export]
macro_rules! account_query_by_key_value {
    ($keyname:expr, $keyvalue:expr) => {
        sqlx::query_as!(
            Account,
            "SELECT id, user_id, created, balance, smps_solved, is_banned, mine_slots, previous_claim, 
            awaiting_claim, username, user_bio, pronouns, avatar_url, next_username_update, is_admin 
            FROM Accounts WHERE ?=?", $keyname, $keyvalue
        )
    };
}

#[macro_export]
macro_rules! account_query_by_key_value_incomplete {
    ($keyname:expr, $keyvalue:expr) => {
        sqlx::query_as!(
            Account,
            "SELECT id, user_id, created, balance, smps_solved, is_banned, mine_slots, previous_claim, 
            awaiting_claim, username, user_bio, pronouns, avatar_url, next_username_update, is_admin 
            FROM Accounts WHERE ? LIKE ?", $keyname, $keyvalue
        )
    };
}

//AHHH WHY DO I HAVE TO WRITE A MACRO TO WRITE A MACRO TO WRITE A MACRO????
#[macro_export]
macro_rules! account_query_by_key {
    ($key:expr, $database:expr) => {
        match $key {
            AccountQueryKey::id(id) => 
            {
                let (key, id) = ($key.as_ref(), &id);
                account_query_by_key_value!(key, id)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchAccount(e))
            },
            AccountQueryKey::user_id(user_id) => 
            {
                let (key, user_id) = ($key.as_ref(), &user_id);
                account_query_by_key_value!(key, user_id)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchAccount(e))
            },
            AccountQueryKey::username(username) =>             
            {
                let (key, username) = ($key.as_ref(), &username);
                account_query_by_key_value!(key, username)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchAccount(e))
            },
            AccountQueryKey::username_incomplete(username_incomplete) =>             
            {
                let (key, username_incomplete) = ($key.as_ref(), &username_incomplete);
                account_query_by_key_value_incomplete!(key, username_incomplete)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchAccount(e))
            },
        }
    };
}
