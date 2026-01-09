use crate::models::{Coverage, EfficiencyEntry, StabilityEntry, StatsResponse};
use alembic_core::Db;
use axum::{extract::State, http::StatusCode, Json};
use sqlx::Row;

pub async fn get_insights(State(db): State<Db>) -> Result<Json<StatsResponse>, StatusCode> {
    // 1. Coverage
    let coverage = sqlx::query(
        r#"
        SELECT 
            (SELECT COUNT(*) FROM raw_events) as total_jobs,
            (SELECT COUNT(DISTINCT hardware_model) FROM raw_events) as unique_hardware
        "#,
    )
    .fetch_one(&db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total_jobs: i64 = coverage.get("total_jobs");
    let unique_hardware: i64 = coverage.get("unique_hardware");

    // 2. Leaderboard
    let leaderboard_rows = sqlx::query(
        r#"
        SELECT hardware_model, encoder, video_codec, resolution, avg_speed, avg_size_reduction_pct, sample_count
        FROM efficiency_stats
        ORDER BY avg_speed DESC
        LIMIT 50
        "#
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let leaderboard = leaderboard_rows
        .into_iter()
        .map(|r| EfficiencyEntry {
            hardware: r.get("hardware_model"),
            encoder: r.get("encoder"),
            codec: r.get("video_codec"),
            res: r.get("resolution"),
            speed: r.get::<Option<f64>, _>("avg_speed").unwrap_or(0.0),
            reduction: r
                .get::<Option<f64>, _>("avg_size_reduction_pct")
                .unwrap_or(0.0),
            samples: r.get::<Option<i64>, _>("sample_count").unwrap_or(0),
        })
        .collect();

    // 3. Stability
    let stability_rows = sqlx::query(
        r#"
        SELECT encoder, error_type, count
        FROM stability_stats
        ORDER BY count DESC
        LIMIT 20
        "#,
    )
    .fetch_all(&db.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stability = stability_rows
        .into_iter()
        .map(|r| StabilityEntry {
            encoder: r.get("encoder"),
            error: r.get("error_type"),
            count: r.get::<Option<i64>, _>("count").unwrap_or(0),
        })
        .collect();

    Ok(Json(StatsResponse {
        schema: 1,
        coverage: Coverage {
            total_jobs,
            unique_hardware,
        },
        leaderboard,
        stability,
    }))
}
