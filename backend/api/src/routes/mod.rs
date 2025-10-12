pub mod timeline;
pub mod user_groups;
pub mod users;

use axum::Router;
use crate::db::DbPool;

pub fn create_routes() -> Router<DbPool> {
    Router::new()
        .nest("/timeline", timeline::routes())
        .nest("/user-groups", user_groups::routes())
        .nest("/users", users::routes())
}
