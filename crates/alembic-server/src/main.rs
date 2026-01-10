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
use tokio::time::interval;
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

    // 0. Load Config
    let config = alembic_core::Config::from_env();
    info!("Starting Alembic on {}...", config.socket_addr());

    // 1. Initialize DB
    let db = Db::new(&config.database_url).await?;
    db.migrate().await?;

    // 2. Start Aggregation Worker
    let db_clone = db.clone();
    let interval_duration = config.aggregation_interval;
    
    tokio::spawn(async move {
        // Initial Aggregation on startup
        if let Err(e) = alembic_aggregate::run_aggregation(&db_clone).await {
             error!("Initial aggregation failed: {}", e);
        }

        let mut interval = interval(interval_duration);
        loop {
            interval.tick().await;
            if let Err(e) = alembic_aggregate::run_aggregation(&db_clone).await {
                error!("Aggregation failed: {}", e);
            }
        }
    });

    // 3. Configure Rate Limiting
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .period(config.rate_limit_period())
            .burst_size(config.rate_limit_burst)
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
    let addr = config.socket_addr();
    info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}
