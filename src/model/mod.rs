pub mod item;
pub mod account;

#[allow(unused)]
pub struct ModelController {
    database: sqlx::SqlitePool,
}


impl ModelController {
    #[allow(unused)]
    pub fn new(database: sqlx::SqlitePool) -> Self {
        Self {
            database,
        }
    }
}