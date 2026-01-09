use crate::models::IngestEvent;
use alembic_core::Db;
use axum::{extract::State, http::StatusCode, Json};
use ulid::Ulid;

pub async fn ingest_event(
    State(db): State<Db>,
    Json(payload): Json<IngestEvent>,
) -> Result<StatusCode, StatusCode> {
    let id = Ulid::new().to_string();

    sqlx::query(
        r#"
        INSERT INTO raw_events (
            id, created_at, app_version, 
            event_type, status, failure_reason,
            hardware_model, encoder,
            duration_ms, input_size_bytes, output_size_bytes, speed_factor,
            video_codec, resolution
        )
        VALUES (?, datetime('now'), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&payload.app_version)
    .bind(&payload.event_type)
    .bind(&payload.status)
    .bind(&payload.failure_reason)
    .bind(&payload.hardware_model)
    .bind(&payload.encoder)
    .bind(&payload.duration_ms)
    .bind(&payload.input_size_bytes)
    .bind(&payload.output_size_bytes)
    .bind(&payload.speed_factor)
    .bind(&payload.video_codec)
    .bind(&payload.resolution)
    .execute(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Insert failed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::ACCEPTED)
}
