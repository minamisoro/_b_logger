use axum::extract::Query;
use axum::{Router, routing::get};

use crate::db::{DbConnection, DbPool};
use crate::response::{ApiError, ApiResponse};

use types::*;
pub mod types {
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};
    use utoipa::{IntoParams, ToSchema};

    use crate::response::IntoSuccess;

    #[derive(Serialize, ToSchema)]
    pub struct TimelinePost {
        /// Unique identifier for the post
        pub id: String,
        /// Post title
        pub title: String,
        /// Content
        pub content: String,
        /// Publication timestamp (ISO 8601)
        pub published_at: String,
    }

    #[derive(Deserialize, ToSchema, IntoParams)]
    pub struct GetTimelineRequest {
        pub limit: u32,
        pub pages: u32,
    }

    #[derive(Serialize, ToSchema)]
    pub struct GetTimelineResponse {
        /// List of published posts
        pub posts: Vec<TimelinePost>,
    }

    impl IntoSuccess for GetTimelineResponse {
        const STATUS_CODE: StatusCode = StatusCode::OK;
    }
}

/// Get timeline of published posts
#[utoipa::path(
    get,
    path = "/api/timeline",
    responses(
        (status = 200, description = "Successfully retrieved timeline", body = GetTimelineResponse),
        (status = 500, description = "Database connection error", body = ApiError)
    ),
    params(GetTimelineRequest),
    tag = "Timeline"
)]
async fn get_timeline(
    DbConnection(_conn): DbConnection,
    Query(query): Query<GetTimelineRequest>,
) -> ApiResponse<GetTimelineResponse> {
    // TODO: Implement database query to fetch published posts
    // For now, returning mock data

    let posts = vec![TimelinePost {
        id: "1".to_string(),
        title: "Sample Post".to_string(),
        content: "Sample Content".to_string(),
        published_at: "2025-10-05T00:00:00Z".to_string(),
    }];

    ApiResponse::Success(GetTimelineResponse { posts })
}

pub fn routes() -> Router<DbPool> {
    Router::new().route("/", get(get_timeline))
}
