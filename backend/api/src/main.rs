use axum::{routing::get, Router};
use blogger_api::{db, openapi::ApiDoc, routes};
use dotenvy::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file if present
    dotenv().ok();

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment variables");

    // Create database pool
    let pool = db::establish_connection_pool(&database_url);

    // Initialize router with database pool state
    let swagger_router = SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi());

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .nest("/api", routes::create_routes())
        .with_state(pool)
        .merge(swagger_router);

    // Set up server address
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server running on http://{}", addr);
    println!("Swagger UI available at http://{}/swagger-ui", addr);

    // Start server
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Blogger API"
}

async fn health_check() -> &'static str {
    "OK"
}
