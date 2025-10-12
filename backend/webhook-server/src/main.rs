use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{env, sync::Arc};
use tracing::{error, info, warn};

/// Configuration for webhook server
#[derive(Clone)]
struct WebhookConfig {
    secret: String,
    deploy_script_path: String,
}

/// GitHub webhook payload for release events
#[derive(Debug, Deserialize)]
struct GitHubReleasePayload {
    action: String,
    release: Release,
    repository: Repository,
}

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    name: Option<String>,
    draft: bool,
    prerelease: bool,
}

#[derive(Debug, Deserialize)]
struct Repository {
    full_name: String,
    clone_url: String,
}

/// Response for webhook endpoint
#[derive(Debug, Serialize)]
struct WebhookResponse {
    message: String,
    success: bool,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "blogger_webhook=info".into()),
        )
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    let webhook_secret = env::var("WEBHOOK_SECRET")
        .expect("WEBHOOK_SECRET must be set");

    let deploy_script_path = env::var("DEPLOY_SCRIPT_PATH")
        .unwrap_or_else(|_| "/app/scripts/deploy.sh".to_string());

    let port = env::var("WEBHOOK_PORT")
        .unwrap_or_else(|_| "9000".to_string())
        .parse::<u16>()
        .expect("WEBHOOK_PORT must be a valid port number");

    let config = Arc::new(WebhookConfig {
        secret: webhook_secret,
        deploy_script_path,
    });

    info!("Webhook secret configured");
    info!("Deploy script path: {}", config.deploy_script_path);

    // Build router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/webhook/github", post(github_webhook_handler))
        .with_state(config);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("Webhook server running on http://{}", addr);
    info!("GitHub webhook endpoint: http://{}/webhook/github", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Blogger Webhook Server"
}

async fn health_check() -> &'static str {
    "OK"
}

/// Validates GitHub webhook signature using HMAC-SHA256
fn validate_github_signature(
    headers: &HeaderMap,
    payload: &[u8],
    secret: &str,
) -> Result<(), String> {
    // Get signature from headers
    let signature_header = headers
        .get("x-hub-signature-256")
        .ok_or("Missing X-Hub-Signature-256 header")?
        .to_str()
        .map_err(|_| "Invalid signature format")?;

    // Extract hex signature (format: "sha256=<hex>")
    let signature_hex = signature_header
        .strip_prefix("sha256=")
        .ok_or("Invalid signature format")?;

    // Decode hex signature
    let expected_signature = hex::decode(signature_hex)
        .map_err(|_| "Invalid signature format")?;

    // Compute HMAC-SHA256
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|_| "Missing secret")?;

    mac.update(payload);

    // Verify signature
    mac.verify_slice(&expected_signature)
        .map_err(|_| "Signature verification failed")?;

    Ok(())
}

/// GitHub webhook handler for release events
async fn github_webhook_handler(
    State(config): State<Arc<WebhookConfig>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    // Validate GitHub signature
    if let Err(e) = validate_github_signature(&headers, &body, &config.secret) {
        warn!("Webhook signature validation failed: {}", e);
        return (
            StatusCode::UNAUTHORIZED,
            Json(WebhookResponse {
                message: format!("Signature validation failed: {}", e),
                success: false,
            }),
        );
    }

    info!("Webhook signature validated successfully");

    // Parse the payload
    let payload: GitHubReleasePayload = match serde_json::from_slice(&body) {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to parse webhook payload: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(WebhookResponse {
                    message: format!("Invalid payload: {}", e),
                    success: false,
                }),
            );
        }
    };

    info!(
        "Received release webhook: action={}, tag={}, repo={}",
        payload.action, payload.release.tag_name, payload.repository.full_name
    );

    // Only process "published" or "released" actions
    if payload.action != "published" && payload.action != "released" {
        info!("Ignoring action: {}", payload.action);
        return (
            StatusCode::OK,
            Json(WebhookResponse {
                message: format!("Ignoring action: {}", payload.action),
                success: true,
            }),
        );
    }

    // Skip draft and prerelease versions
    if payload.release.draft || payload.release.prerelease {
        info!("Ignoring draft or prerelease");
        return (
            StatusCode::OK,
            Json(WebhookResponse {
                message: "Ignoring draft or prerelease".to_string(),
                success: true,
            }),
        );
    }

    // Execute deployment script
    info!(
        "Triggering deployment for release {}",
        payload.release.tag_name
    );

    match execute_deployment(&config.deploy_script_path, &payload.release.tag_name).await {
        Ok(output) => {
            info!("Deployment completed successfully");
            info!("Output: {}", output);
            (
                StatusCode::OK,
                Json(WebhookResponse {
                    message: format!(
                        "Deployment triggered successfully for tag {}",
                        payload.release.tag_name
                    ),
                    success: true,
                }),
            )
        }
        Err(e) => {
            error!("Deployment failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WebhookResponse {
                    message: format!("Deployment failed: {}", e),
                    success: false,
                }),
            )
        }
    }
}

/// Execute the deployment script
async fn execute_deployment(script_path: &str, tag: &str) -> Result<String, String> {
    use tokio::process::Command;

    info!("Executing deployment script: {}", script_path);

    let output = Command::new(script_path)
        .arg(tag)
        .output()
        .await
        .map_err(|e| format!("Failed to execute deployment script: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Deployment script failed: {}", stderr))
    }
}
