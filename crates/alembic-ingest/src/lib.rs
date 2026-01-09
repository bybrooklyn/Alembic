pub mod models;
pub mod handlers;

use axum::{routing::post, Router};
use alembic_core::Db;
use handlers::ingest_event;

pub fn router(db: Db) -> Router {
    Router::new()
        .route("/v1/event", post(ingest_event))
        .with_state(db)
}
