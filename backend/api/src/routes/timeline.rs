use axum::{extract::State, http::StatusCode, routing::get, Router};
use serde::Serialize;

use crate::db::DbPool;
use crate::response::{ApiResponse, IntoSuccess};

#[derive(Serialize)]
pub struct TimelinePost {
    pub id: String,
    pub title: String,
    pub excerpt: String,
    pub published_at: String,
}

#[derive(Serialize)]
pub struct TimelineResponse {
    pub posts: Vec<TimelinePost>,
}

impl IntoSuccess for TimelineResponse {
    const STATUS_CODE: StatusCode = StatusCode::OK;
}

/// Get timeline of published posts
async fn get_timeline(State(pool): State<DbPool>) -> ApiResponse<TimelineResponse> {
    // TODO: Implement database query to fetch published posts
    // For now, returning mock data
    let _conn = pool.get().expect("Failed to get database connection");

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
