use axum::{http::Request, routing::get, Router};
use blogger_api::{config::AppConfig, db, middleware::UuidV7RequestId, openapi::ApiDoc, routes};
use dotenvy::dotenv;
use std::env;
use tower_http::{
    cors::CorsLayer,
    request_id::{PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::{MakeSpan, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Custom span maker that includes request ID
#[derive(Clone)]
struct RequestIdSpan;

impl<B> MakeSpan<B> for RequestIdSpan {
    fn make_span(&mut self, request: &Request<B>) -> tracing::Span {
        let request_id = request
            .extensions()
            .get::<RequestId>()
            .and_then(|id| id.header_value().to_str().ok())
            .unwrap_or("unknown");

        tracing::span!(
            Level::INFO,
            "request",
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
            request_id = %request_id,
        )
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "blogger_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables from .env file if present
    dotenv().ok();

    // Load configuration from config.toml
    let config = AppConfig::load()
        .expect("Failed to load configuration from config.toml");

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment variables");

    // Create database pool
    let pool = db::establish_connection_pool(&database_url);

    // Configure CORS with allowed origins from config
    let cors_origins: Vec<_> = config
        .cors_origins()
        .into_iter()
        .map(|origin| origin.parse().expect("Invalid CORS origin"))
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(cors_origins)
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::PATCH,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ]);

    // Initialize router with database pool state
    let swagger_router = SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi());

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .nest("/api", routes::create_routes())
        .with_state(pool)
        .merge(swagger_router)
        // Add CORS layer (runs outermost)
        .layer(cors)
        // Add trace layer with custom span that includes request ID
        .layer(TraceLayer::new_for_http().make_span_with(RequestIdSpan))
        // Add request ID propagation to response headers
        .layer(PropagateRequestIdLayer::new(
            "x-request-id".parse().unwrap(),
        ))
        // Add request ID generation (runs first, sets the ID)
        .layer(SetRequestIdLayer::new(
            "x-request-id".parse().unwrap(),
            UuidV7RequestId::default(),
        ));

    // Set up server address from config
    let addr = config.server_address();
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Server running on http://{}", addr);
    tracing::info!("Swagger UI available at http://{}/swagger-ui", addr);

    // Start server
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Blogger API"
}

async fn health_check() -> &'static str {
    "OK"
}
