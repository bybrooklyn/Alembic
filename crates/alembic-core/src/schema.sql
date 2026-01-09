-- Raw Events (Append Only)
CREATE TABLE IF NOT EXISTS raw_events (
    id TEXT PRIMARY KEY,          
    created_at DATETIME NOT NULL,
    app_version TEXT NOT NULL,
    
    -- Event Context
    event_type TEXT NOT NULL,     -- 'job_started', 'job_finished'
    status TEXT,                  -- 'success', 'failure'
    failure_reason TEXT,          -- 'OOM', 'Init', 'Unsupported', 'Timeout'
    
    -- Hardware Context
    hardware_model TEXT,          
    encoder TEXT,                 
    
    -- Metrics
    duration_ms INTEGER,
    input_size_bytes INTEGER,
    output_size_bytes INTEGER,
    speed_factor REAL,

    -- Dimensions
    video_codec TEXT,
    resolution TEXT
);

CREATE INDEX IF NOT EXISTS idx_raw_events_created_at ON raw_events(created_at);

-- Efficiency Stats (Leaderboard)
CREATE TABLE IF NOT EXISTS efficiency_stats (
    hardware_model TEXT NOT NULL,
    encoder TEXT NOT NULL,
    video_codec TEXT NOT NULL,
    resolution TEXT NOT NULL,
    
    sample_count INTEGER DEFAULT 0,
    avg_speed REAL DEFAULT 0.0,
    avg_size_reduction_pct REAL DEFAULT 0.0,
    success_rate REAL DEFAULT 0.0,

    PRIMARY KEY (hardware_model, encoder, video_codec, resolution)
);

-- Stability Stats (Failures)
CREATE TABLE IF NOT EXISTS stability_stats (
    encoder TEXT NOT NULL,
    error_type TEXT NOT NULL,
    count INTEGER DEFAULT 0,
    PRIMARY KEY (encoder, error_type)
);
