use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use std::sync::Arc;

pub struct AppState {
    pub pool: Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
}
