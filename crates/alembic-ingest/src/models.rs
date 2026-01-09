use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IngestEvent {
    pub app_version: String,
    
    pub event_type: String, // job_started, job_finished
    pub status: Option<String>, // success, failure
    pub failure_reason: Option<String>,
    
    pub hardware_model: Option<String>,
    pub encoder: Option<String>,
    
    // Metrics
    pub duration_ms: Option<i64>,
    pub input_size_bytes: Option<i64>,
    pub output_size_bytes: Option<i64>,
    pub speed_factor: Option<f64>,

    // Dimensions
    pub video_codec: Option<String>,
    pub resolution: Option<String>,
}
