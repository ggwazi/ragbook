//! Ragbook - Axum-based documentation server with markdown rendering

use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod markdown;

/// Application entry point
#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ragbook=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build the application router
    let app = create_router();

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Starting server at http://{}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

/// Create the application router with all routes
pub fn create_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/docs/*path", get(docs_handler))
        .route("/health", get(health_handler))
        .nest_service("/static", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
}

/// Index page handler
async fn index_handler() -> impl IntoResponse {
    Html(include_str!("../templates/index.html"))
}

/// Health check endpoint
async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

/// Documentation page handler - renders markdown files
async fn docs_handler(Path(path): Path<String>) -> impl IntoResponse {
    let docs_path = format!("docs/{}", path);

    match tokio::fs::read_to_string(&docs_path).await {
        Ok(content) => {
            let html_content = markdown::render_markdown(&content);
            let page = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ragbook - Documentation</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 800px; margin: 0 auto; padding: 2rem; line-height: 1.6; }}
        pre {{ background: #f4f4f4; padding: 1rem; overflow-x: auto; border-radius: 4px; }}
        code {{ background: #f4f4f4; padding: 0.2rem 0.4rem; border-radius: 2px; }}
        h1, h2, h3 {{ color: #333; }}
        a {{ color: #0066cc; }}
    </style>
</head>
<body>
    <nav><a href="/">← Home</a></nav>
    <main>{}</main>
</body>
</html>"#,
                html_content
            );
            (StatusCode::OK, Html(page))
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Html("<h1>404 - Document Not Found</h1>".to_string()),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_index_endpoint() {
        let app = create_router();
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
