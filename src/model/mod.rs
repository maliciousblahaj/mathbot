pub mod item;
pub mod account;

pub struct ModelController {
    database: sqlx::SqlitePool,
}

impl ModelController {
    pub fn new(database: sqlx::SqlitePool) -> Self {
        Self {
            database,
        }
    }
}
