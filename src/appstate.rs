use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub pool: Arc<Mutex<r2d2::Pool<ConnectionManager<SqliteConnection>>>>,
}
