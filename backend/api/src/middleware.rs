use axum::{
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use tower_http::request_id::{MakeRequestId, RequestId};
use tracing::Instrument;
use uuid::Uuid;

/// Request ID generator using UUID v7
#[derive(Clone, Default)]
pub struct UuidV7RequestId;

impl MakeRequestId for UuidV7RequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::now_v7().to_string();
        let header_value = HeaderValue::from_str(&request_id).ok()?;
        Some(RequestId::new(header_value))
    }
}

/// Middleware to create a span with request ID for all logs in the request context
pub async fn request_span(request: Request, next: Next) -> Response {
    // Get request ID from extension (set by SetRequestIdLayer)
    let request_id = request
        .extensions()
        .get::<RequestId>()
        .and_then(|id| id.header_value().to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let method = request.method().clone();
    let uri = request.uri().clone();

    // Create a span that will be the parent of all logs in this request
    let span = tracing::info_span!(
        "request",
        request_id = %request_id,
        method = %method,
        uri = %uri,
    );

    // Execute the request within the span context
    async move {
        tracing::info!("incoming request");
        let response = next.run(request).await;
        tracing::info!(status = %response.status(), "request completed");
        response
    }
    .instrument(span)
    .await
}
