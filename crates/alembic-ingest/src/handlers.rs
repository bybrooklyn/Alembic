use axum::{extract::State, http::StatusCode, Json};
use alembic_core::Db;
use crate::models::IngestEvent;
use ulid::Ulid;

pub async fn ingest_event(
    State(db): State<Db>,
    Json(payload): Json<IngestEvent>,
) -> Result<StatusCode, StatusCode> {
    
    let id = Ulid::new().to_string();
    
    sqlx::query!(
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
        id,
        payload.app_version,
        payload.event_type,
        payload.status,
        payload.failure_reason,
        payload.hardware_model,
        payload.encoder,
        payload.duration_ms,
        payload.input_size_bytes,
        payload.output_size_bytes,
        payload.speed_factor,
        payload.video_codec,
        payload.resolution
    )
    .execute(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Insert failed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::ACCEPTED)
}
