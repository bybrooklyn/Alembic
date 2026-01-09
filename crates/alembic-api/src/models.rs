use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsResponse {
    pub schema: i32,
    pub coverage: Coverage,
    pub leaderboard: Vec<EfficiencyEntry>,
    pub stability: Vec<StabilityEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coverage {
    pub total_jobs: i64,
    pub unique_hardware: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EfficiencyEntry {
    pub hardware: String,
    pub encoder: String,
    pub codec: String,
    pub res: String,
    pub speed: f64,
    pub reduction: f64,
    pub samples: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StabilityEntry {
    pub encoder: String,
    pub error: String,
    pub count: i64,
}
