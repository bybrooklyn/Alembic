use alembic_core::{Db, DbError};
use tracing::info;

pub async fn run_aggregation(db: &Db) -> Result<(), DbError> {
    info!("Starting aggregation (Reframed)...");

    let mut tx = db.pool.begin().await.map_err(|e| DbError::Connection(e))?;

    // 1. Efficiency Stats (The Leaderboard)
    // Clear old comparisons for MVP simplicity
    sqlx::query("DELETE FROM efficiency_stats")
        .execute(&mut *tx)
        .await
        .map_err(|e| DbError::Connection(e))?;

    sqlx::query(
        r#"
        INSERT INTO efficiency_stats (
            hardware_model, encoder, video_codec, resolution,
            sample_count, avg_speed, avg_size_reduction_pct, success_rate
        )
        SELECT 
            hardware_model,
            encoder,
            video_codec,
            resolution,
            COUNT(*) as sample_count,
            AVG(speed_factor) as avg_speed,
            AVG(CAST((input_size_bytes - output_size_bytes) AS REAL) / input_size_bytes) as avg_size_reduction_pct,
            (CAST(SUM(CASE WHEN status = 'success' THEN 1 ELSE 0 END) AS REAL) / COUNT(*)) as success_rate
        FROM raw_events
        WHERE event_type = 'job_finished' 
          AND hardware_model IS NOT NULL 
          AND encoder IS NOT NULL
          AND video_codec IS NOT NULL
          AND resolution IS NOT NULL
        GROUP BY hardware_model, encoder, video_codec, resolution
        "#
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| DbError::Connection(e))?;

    // 2. Stability Stats (The Failure Report)
    sqlx::query("DELETE FROM stability_stats")
        .execute(&mut *tx)
        .await
        .map_err(|e| DbError::Connection(e))?;

    sqlx::query(
        r#"
        INSERT INTO stability_stats (encoder, error_type, count)
        SELECT 
            encoder,
            failure_reason as error_type,
            COUNT(*) as count
        FROM raw_events
        WHERE status = 'failure'
          AND failure_reason IS NOT NULL
          AND encoder IS NOT NULL
        GROUP BY encoder, failure_reason
        "#,
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| DbError::Connection(e))?;

    tx.commit().await.map_err(|e| DbError::Connection(e))?;

    info!("Aggregation complete.");
    Ok(())
}
