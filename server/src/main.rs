use axum::{
    extract::{Json, State},
    http::{StatusCode, Uri},
    response::{Html, Json as JsonResponse, Response, IntoResponse},
    routing::{get, post},
    Router,
};
use common::SystemInfo;
use dashmap::DashMap;
use rust_embed::RustEmbed;
use std::sync::Arc;
use tracing::info;
// use tower_http::services::ServeDir; // removed unused import

/// Embed static files into the binary
#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

/// Application state to store metrics from all devices
struct AppState {
    metrics: DashMap<String, SystemInfo>,
}

/// Handler function to receive metrics from agents
async fn receive_metrics(
    State(state): State<Arc<AppState>>,
    Json(system_info): Json<SystemInfo>,
) -> (StatusCode, &'static str) {
    info!("Received metrics from device: {}", system_info.device_id);
    info!("OS: {}, CPU: {:.1}%, RAM: {}/{} MB", 
        system_info.os_info, 
        system_info.cpu_usage, 
        system_info.ram_used_mb, 
        system_info.ram_total_mb
    );
    info!("Last seen: {}", system_info.last_seen);
    info!("---");
    
    // Store or update the metrics in memory
    state.metrics.insert(system_info.device_id.clone(), system_info);
    
    (StatusCode::OK, "Veri Alındı")
}

/// Handler function to get all metrics
async fn get_all_metrics(State(state): State<Arc<AppState>>) -> JsonResponse<Vec<SystemInfo>> {
    let metrics: Vec<SystemInfo> = state.metrics.iter().map(|entry| entry.value().clone()).collect();
    JsonResponse(metrics)
}

/// Handler function to serve embedded static files
async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    
    if path.is_empty() || path == "index.html" {
        // Serve index.html for root path
        match Assets::get("index.html") {
            Some(content) => {
                let html = String::from_utf8_lossy(&content.data);
                Html(html.to_string()).into_response()
            }
            None => (StatusCode::NOT_FOUND, "File not found").into_response()
        }
    } else {
        // Serve other static files
        match Assets::get(path) {
            Some(content) => {
                let mime_type = if path.ends_with(".css") {
                    "text/css"
                } else if path.ends_with(".js") {
                    "application/javascript"
                } else {
                    "text/plain"
                };
                
                Response::builder()
                    .header("Content-Type", mime_type)
                    .body(axum::body::Body::from(content.data.to_vec()))
                    .unwrap()
            }
            None => (StatusCode::NOT_FOUND, "File not found").into_response()
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter("server=info")
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
    
    info!("Server starting on 0.0.0.0:3000...");
    
    // Create application state
    let state = Arc::new(AppState {
        metrics: DashMap::new(),
    });
    
    // Create the router with the metrics endpoints and static file serving
    let app = Router::new()
        .route("/api/metrics", post(receive_metrics))
        .route("/api/all_metrics", get(get_all_metrics))
        .route("/", get(static_handler)) // Serve index.html at root
        .route("/*path", get(static_handler)) // Serve all other static files
        .with_state(state);
    
    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server is running on http://0.0.0.0:3000");
    info!("Available endpoints:");
    info!("  POST /api/metrics - Receive metrics from agents");
    info!("  GET  /api/all_metrics - Get all stored metrics");
    info!("  GET  /static/* - Serve static files (e.g., index.html, styles.css, script.js)");
    
    axum::serve(listener, app).await.unwrap();
} 