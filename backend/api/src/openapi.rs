use utoipa::OpenApi;

use crate::response::ApiError;
use crate::routes::timeline::types::{GetTimelineRequest, GetTimelineResponse, TimelinePost};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::timeline::get_timeline,
    ),
    components(
        schemas(ApiError, TimelinePost, GetTimelineResponse)
    ),
    tags(
        (name = "Timeline", description = "Timeline endpoints for retrieving published posts")
    ),
    info(
        title = "Blogger API",
        version = "0.1.0",
        description = "REST API for the blog platform",
    )
)]
pub struct ApiDoc;
