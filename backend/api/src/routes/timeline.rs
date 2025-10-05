use axum::{http::StatusCode, routing::get, Router};
use serde::Serialize;
use utoipa::ToSchema;

use crate::db::{DbConnection, DbPool};
use crate::response::{ApiError, ApiResponse, IntoSuccess};

#[derive(Serialize, ToSchema)]
pub struct TimelinePost {
    /// Unique identifier for the post
    pub id: String,
    /// Post title
    pub title: String,
    /// Post excerpt/summary
    pub excerpt: String,
    /// Publication timestamp (ISO 8601)
    pub published_at: String,
}

#[derive(Serialize, ToSchema)]
pub struct TimelineResponse {
    /// List of published posts
    pub posts: Vec<TimelinePost>,
}

impl IntoSuccess for TimelineResponse {
    const STATUS_CODE: StatusCode = StatusCode::OK;
}

/// Get timeline of published posts
#[utoipa::path(
    get,
    path = "/api/timeline",
    responses(
        (status = 200, description = "Successfully retrieved timeline", body = TimelineResponse),
        (status = 500, description = "Database connection error", body = ApiError)
    ),
    tag = "Timeline"
)]
async fn get_timeline(DbConnection(_conn): DbConnection) -> ApiResponse<TimelineResponse> {
    // TODO: Implement database query to fetch published posts
    // For now, returning mock data

    let posts = vec![TimelinePost {
        id: "1".to_string(),
        title: "Sample Post".to_string(),
        excerpt: "This is a sample post".to_string(),
        published_at: "2025-10-05T00:00:00Z".to_string(),
    }];

    ApiResponse::Success(TimelineResponse { posts })
}

pub fn routes() -> Router<DbPool> {
    Router::new().route("/", get(get_timeline))
}
