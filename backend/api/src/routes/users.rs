use axum::{Router, routing::get};
use diesel::prelude::*;

use crate::db::{DbConnection, DbPool};
use crate::response::{ApiError, ApiResponse};

use types::*;
pub mod types {
    use axum::http::StatusCode;
    use serde::Serialize;
    use utoipa::ToSchema;

    use crate::response::IntoSuccess;

    #[derive(Serialize, ToSchema)]
    pub struct UserInfo {
        /// User's unique identifier
        pub id: String,
        /// Username
        pub username: String,
        /// Display name
        pub display_name: Option<String>,
    }

    impl IntoSuccess for UserInfo {
        const STATUS_CODE: StatusCode = StatusCode::OK;
    }
}

/// Get a random user (for testing/demo purposes)
#[utoipa::path(
    get,
    path = "/api/users/random",
    responses(
        (status = 200, description = "Successfully retrieved random user", body = UserInfo),
        (status = 500, description = "Database connection error", body = ApiError)
    ),
    tag = "Users"
)]
async fn get_random_user(
    DbConnection(mut conn): DbConnection,
) -> ApiResponse<UserInfo> {
    use crate::schema::users::dsl::*;
    use diesel::dsl::sql;
    use diesel::sql_types::BigInt;

    // Get a random user using RANDOM()
    let random_user = match users
        .order(sql::<BigInt>("RANDOM()"))
        .select((id, username, display_name))
        .first::<(uuid::Uuid, String, Option<String>)>(&mut conn)
    {
        Ok(user) => user,
        Err(e) => {
            return ApiResponse::Failure(ApiError {
                status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "DATABASE_ERROR".to_string(),
                message: e.to_string(),
            });
        }
    };

    ApiResponse::Success(UserInfo {
        id: random_user.0.to_string(),
        username: random_user.1,
        display_name: random_user.2,
    })
}

pub fn routes() -> Router<DbPool> {
    Router::new().route("/random", get(get_random_user))
}
