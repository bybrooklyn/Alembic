use alembic_core::Db;
use axum::{
    extract::DefaultBodyLimit,
    http::{header, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router,
};
use rust_embed::RustEmbed;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tower_http::cors::CorsLayer;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tracing::{info, error};

#[derive(RustEmbed)]
#[folder = "../../web/dist"]
struct Assets;

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.is_empty() {
        path = "index.html".to_string();
    }

    match Assets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return StatusCode::NOT_FOUND.into_response();
            }
            // Fallback to index.html for SPA routing
            match Assets::get("index.html") {
                Some(content) => {
                    let mime = mime_guess::from_path("index.html").first_or_octet_stream();
                    ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
                }
                None => StatusCode::NOT_FOUND.into_response(),
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    info!("Starting Alembic...");

    // 1. Initialize DB
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:alembic.db".to_string());
    let db = Db::new(&db_url).await?;
    db.migrate().await?;

    // 2. Start Aggregation Worker
    let db_clone = db.clone();
    tokio::spawn(async move {
        // Initial Aggregation on startup
        if let Err(e) = alembic_aggregate::run_aggregation(&db_clone).await {
             error!("Initial aggregation failed: {}", e);
        }

        let mut interval = interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            if let Err(e) = alembic_aggregate::run_aggregation(&db_clone).await {
                error!("Aggregation failed: {}", e);
            }
        }
    });

    // 3. Configure Rate Limiting
    // 30 requests per minute = 1 request every 2 seconds.
    // Burst size = 10.
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .period(Duration::from_secs(2)) // 1 request every 2 seconds = 30 req/min
            .burst_size(10)
            .finish()
            .unwrap(),
    );

    // 4. Build Router
    let app = Router::new()
        // API
        .nest("/api", alembic_api::router(db.clone()))
        // Ingest
        .nest("/", alembic_ingest::router(db.clone()))
        // Global Layers
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(16384)) // 16KB Max Body
        .layer(GovernorLayer {
            config: governor_conf,
        })
        .route("/health", get(|| async { "OK" }))
        // Static Assets (Fallback)
        .fallback(static_handler);

    // 5. Start Server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}
