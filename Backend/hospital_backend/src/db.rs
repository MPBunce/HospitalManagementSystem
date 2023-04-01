use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub struct AppState {
    pub db_pool: Pool<SqliteConnectionManager>,
}

//use serde::{Deserialize, Serialize};
pub fn create_pool(database_path: String) -> Pool<SqliteConnectionManager> {
    let manager = SqliteConnectionManager::file(database_path);
    Pool::builder().max_size(10).build(manager).unwrap()
}

//Im Testing commits form linux