#[macro_export]
macro_rules! account_query_by_key_user_id {
    ($keyvalue:expr) => {
        sqlx::query_as!(
            Account,
            "SELECT id, user_id, created, balance, smps_solved, banned, mine_slots, previous_claim, 
            awaiting_claim, username, user_bio, pronouns, avatar_url, next_username_update, is_admin 
            FROM Accounts WHERE user_id=?",  $keyvalue
        )
    };
}

#[macro_export]
macro_rules! account_query_by_key_id {
    ($keyvalue:expr) => {
        sqlx::query_as!(
            Account,
            "SELECT id, user_id, created, balance, smps_solved, banned, mine_slots, previous_claim, 
            awaiting_claim, username, user_bio, pronouns, avatar_url, next_username_update, is_admin 
            FROM Accounts WHERE id=?",  $keyvalue
        )
    };
}

#[macro_export]
macro_rules! account_query_by_key_username {
    ($keyvalue:expr) => {
        sqlx::query_as!(
            Account,
            "SELECT id, user_id, created, balance, smps_solved, banned, mine_slots, previous_claim, 
            awaiting_claim, username, user_bio, pronouns, avatar_url, next_username_update, is_admin 
            FROM Accounts WHERE username=? COLLATE NOCASE",  $keyvalue
        )
    };
}

#[macro_export]
macro_rules! account_query_by_key_username_incomplete {
    ($keyvalue:expr) => {
        sqlx::query_as!(
            Account,
            "SELECT id, user_id, created, balance, smps_solved, banned, mine_slots, previous_claim, 
            awaiting_claim, username, user_bio, pronouns, avatar_url, next_username_update, is_admin 
            FROM Accounts WHERE username LIKE ? COLLATE NOCASE", $keyvalue
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
                account_query_by_key_id!(id)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchAccount(e))
            },
            AccountQueryKey::user_id(user_id) => 
            {
                account_query_by_key_user_id!(user_id)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchAccount(e))
            },
            AccountQueryKey::username(username) =>             
            {
                account_query_by_key_username!(username)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchAccount(e))
            },
            AccountQueryKey::username_incomplete(username_incomplete) =>             
            {
                account_query_by_key_username_incomplete!(username_incomplete)
                    .fetch_one($database)
                    .await
                    .map_err(|e| Error::FailedToFetchAccount(e))
            },
        }
    };
}
