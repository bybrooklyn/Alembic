pub mod models;
pub mod handlers;

use axum::{routing::get, Router};
use alembic_core::Db;
use handlers::get_insights;

pub fn router(db: Db) -> Router {
    Router::new()
        .route("/v1/stats/insights", get(get_insights))
        .with_state(db)
}
