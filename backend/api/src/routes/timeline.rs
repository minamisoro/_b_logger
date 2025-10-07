use axum::extract::Query;
use axum::{Router, routing::get};
use diesel::prelude::*;

use crate::db::{DbConnection, DbPool};
use crate::response::{ApiError, ApiResponse};

use types::*;
pub mod types {
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};
    use utoipa::{IntoParams, ToSchema};
    use uuid::Uuid;

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
        /// Post category (derived from primary tag)
        pub category: String,
    }

    #[derive(Deserialize, ToSchema, IntoParams)]
    pub struct GetTimelineRequest {
        /// Number of posts per page (default: 20, max: 100)
        #[serde(default = "default_limit")]
        pub limit: i64,
        /// Cursor for pagination (use last post's published_at timestamp)
        pub cursor: Option<String>,
        /// Filter by specific user ID
        pub user_id: Option<Uuid>,
        /// Filter by user group ID
        pub group_id: Option<Uuid>,
    }

    fn default_limit() -> i64 {
        20
    }

    #[derive(Serialize, ToSchema)]
    pub struct GetTimelineResponse {
        /// List of published posts
        pub posts: Vec<TimelinePost>,
        /// Cursor for next page (null if no more posts)
        pub next_cursor: Option<String>,
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
    DbConnection(mut conn): DbConnection,
    Query(query): Query<GetTimelineRequest>,
) -> ApiResponse<GetTimelineResponse> {
    use crate::models::Post;
    use crate::schema::posts::dsl::*;
    use crate::schema::user_group_members;

    // Limit to max 100 posts per request
    let limit = query.limit.min(100);

    // Build base query for published posts
    let mut posts_query = posts
        .filter(published.eq(true))
        .filter(visibility.eq("public"))
        .into_boxed();

    // Apply cursor-based pagination if cursor is provided
    if let Some(cursor) = &query.cursor {
        if let Ok(cursor_time) = chrono::DateTime::parse_from_rfc3339(cursor) {
            posts_query = posts_query.filter(published_at.lt(cursor_time.naive_utc()));
        }
    }

    // Apply user filtering
    if let Some(user_id) = query.user_id {
        posts_query = posts_query.filter(author_id.eq(user_id));
    } else if let Some(group_id) = query.group_id {
        // Filter by user group members
        let user_ids: Vec<uuid::Uuid> = match user_group_members::table
            .filter(user_group_members::group_id.eq(group_id))
            .select(user_group_members::user_id)
            .load(&mut conn)
        {
            Ok(ids) => ids,
            Err(e) => {
                return ApiResponse::Failure(ApiError {
                    status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    code: "DATABASE_ERROR".to_string(),
                    message: e.to_string(),
                });
            }
        };

        posts_query = posts_query.filter(author_id.eq_any(user_ids));
    }

    // Execute query with ordering and limit
    let result_posts: Vec<Post> = match posts_query
        .order(published_at.desc())
        .limit(limit + 1) // Fetch one extra to determine if there are more pages
        .load(&mut conn)
    {
        Ok(posts_vec) => posts_vec,
        Err(e) => {
            return ApiResponse::Failure(ApiError {
                status_code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "DATABASE_ERROR".to_string(),
                message: e.to_string(),
            });
        }
    };

    // Determine next cursor
    let has_more = result_posts.len() > limit as usize;
    let posts_to_return = if has_more {
        &result_posts[..limit as usize]
    } else {
        &result_posts
    };

    let next_cursor = if has_more {
        posts_to_return.last().and_then(|p| p.published_at.map(|t| t.and_utc().to_rfc3339()))
    } else {
        None
    };

    // Convert posts to timeline format
    // For now, using a default category - will be enhanced with tag-based categories
    let timeline_posts: Vec<TimelinePost> = posts_to_return
        .iter()
        .map(|p| TimelinePost {
            id: p.id.to_string(),
            title: p.title.clone(),
            content: p.content.clone(),
            published_at: p.published_at
                .map(|t| t.and_utc().to_rfc3339())
                .unwrap_or_default(),
            category: "generic".to_string(), // TODO: Derive from tags
        })
        .collect();

    ApiResponse::Success(GetTimelineResponse {
        posts: timeline_posts,
        next_cursor,
    })
}

pub fn routes() -> Router<DbPool> {
    Router::new().route("/", get(get_timeline))
}
