pub mod timeline;

use axum::Router;
use crate::db::DbPool;

pub fn create_routes() -> Router<DbPool> {
    Router::new()
        .nest("/timeline", timeline::routes())
}
