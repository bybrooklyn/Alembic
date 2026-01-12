# Alembic

**Alembic** is the open-source community "Argument Engine" for media transcoding.

It exists to answer one question: **"What is the best way to transcode video?"**

Unlike traditional stats dashboards which focus on vanity metrics ("Total Events"), Alembic focuses entirely on **Efficiency** and **Stability**. It aggregates anonymous performance data from thousands of [Alchemist](https://github.com/bybrooklyn/alchemist) nodes to build a transparent, defensible leaderboard of hardware and software encoders.

---

##  Data Trust & Privacy
Alembic is built on a foundation of absolute anonymity. We believe infrastructure tools should be boring and trustworthy.

*   **What we collect**: Hardware Model (e.g., "RTX 3070 TI, Ryzen 5 9700X"), Encoder Settings (e.g., "av1_nvenc, av1_qsv"), Duration, Size Reduction %, Status (Success/Fail).
*   **What we DO NOT collect**: IP Addresses, Filenames, File Paths, Media Titles, User IDs, Persistent Session IDs.
*   **Storage**: Data is stored in a local SQLite database (`alembic.db`). You control it. You can inspect it. You can delete it.

---

##  Architecture

Alembic is a high-performance, single-binary application.

### Backend (Rust)
Built as a workspace of crates for clean separation of concerns:
*   `alembic-server`: Main Axum entrypoint. Embeds the UI and handles rate limiting.
*   `alembic-ingest`: Handles data intake. Validates schema, ensures anonymity, and writes to an append-only log.
*   `alembic-aggregate`: Background worker. Periodically computes "Leaderboards" and "Stability Reports" from raw events.
*   `alembic-api`: Read-only public API for fetching aggregated insights.
*   `alembic-core`: Shared database logic (SQLx/SQLite) and configuration.

### Frontend (Astro + React)
A static SPA bundled directly into the Rust binary.
*   **Framework**: Astro 5 (for build) + React (for dashboard interactivity).
*   **Styling**: Tailwind CSS with the "Helios" design system (Dark, Warm, Industrial).
*   **Charts**: Custom efficiency bars and stability metrics.

---

##  Getting Started

### Prerequisites
*   **Rust** (latest stable)
*   **Bun** (for frontend build)
*   **SQLite** (runtime library)

### Development Setup

1.  **Clone the repository**
    ```bash
    git clone https://github.com/bybrooklyn/alembic.git
    cd alembic
    ```

2.  **Build the Frontend**
    ```bash
    cd web
    bun install
    bun run build
    cd ..
    ```

3.  **Run the Server**
    The server will automatically create `alembic.db` and apply migrations.
    ```bash
    cargo run -p alembic-server
    ```

4.  **View Dashboard**
    Open `http://localhost:3000` in your browser.

### Docker Deployment

1.  **Run with Docker Compose**
    ```bash
    docker-compose up -d
    ```

    The service will be available at `http://localhost:3000`. Data will be persisted in the `./data` directory.

---

##  API Reference

### Base URL
Local development uses `http://localhost:3000`. The API is mounted under `/api` for read-only endpoints and `/v1` for ingest.

### Health
`GET /health`
Returns `OK` when the server is up.

### Ingest
`POST /v1/event`
Submit a telemetry event for aggregation.
*   **Rate Limit**: Configurable via `ALEMBIC_RATE_LIMIT_PER_MIN` (default 30/min) and `ALEMBIC_RATE_LIMIT_BURST` (default 10).
*   **Payload**: JSON, max 16KB.
*   **Response**: `202 Accepted` on success.
*   **Errors**: `429 Too Many Requests` when rate limited, `413 Payload Too Large` when over the size limit.

```json
{
  "app_version": "1.0.0",
  "event_type": "job_finished",
  "status": "success", 
  "hardware_model": "Intel Arc A380",
  "encoder": "av1_qsv",
  "video_codec": "av1",
  "resolution": "1080p",
  "duration_ms": 2500,
  "input_size_bytes": 1024000,
  "output_size_bytes": 600100,
  "speed_factor": 2.65
}
```

Fields:
*   `app_version` (string, required): Version of the client submitting the event.
*   `event_type` (string, required): `job_started` or `job_finished`.
*   `status` (string, optional): `success` or `failure`.
*   `failure_reason` (string, optional): Free-form error category when `status` is `failure`.
*   `hardware_model` (string, optional): GPU/CPU model label.
*   `encoder` (string, optional): Encoder key, e.g. `av1_qsv`, `hevc_nvenc`.
*   `duration_ms` (number, optional): Encode duration in milliseconds.
*   `input_size_bytes` (number, optional): Input size in bytes.
*   `output_size_bytes` (number, optional): Output size in bytes.
*   `speed_factor` (number, optional): Relative speed multiplier (higher is faster).
*   `video_codec` (string, optional): Output codec identifier, e.g. `av1`.
*   `resolution` (string, optional): Output resolution, e.g. `1080p`.

### Insights
`GET /api/v1/stats/insights`
Get the efficiency leaderboard and stability report.
*   **Response**: `200 OK` with aggregated stats.

```json
{
  "schema": 1,
  "coverage": { "total_jobs": 15420, "unique_hardware": 12 },
  "leaderboard": [ ... ],
  "stability": [ ... ]
}
```

Response details:
*   `coverage.total_jobs`: Count of ingested jobs.
*   `coverage.unique_hardware`: Distinct hardware models seen.
*   `leaderboard[]`: Up to 50 entries ordered by `speed` desc.
*   `leaderboard[].hardware`: Hardware model.
*   `leaderboard[].encoder`: Encoder name.
*   `leaderboard[].codec`: Output codec.
*   `leaderboard[].res`: Resolution bucket.
*   `leaderboard[].speed`: Average speed factor.
*   `leaderboard[].reduction`: Average size reduction percent.
*   `leaderboard[].samples`: Sample count for the entry.
*   `stability[]`: Up to 20 entries ordered by `count` desc.
*   `stability[].encoder`: Encoder name.
*   `stability[].error`: Error category.
*   `stability[].count`: Occurrence count.
---

##  Contributing

Alembic is open source (MIT). We welcome contributions that improve data accuracy, add new comparison dimensions, or enhance the dashboard's "Argument Engine" capabilities.

**Rules:**
1.  **No PII**: Code that attempts to log IPs or persistent IDs will be rejected.
2.  **Performance First**: The ingest path must remain blazing fast.
3.  **Boring is Good**: Prefer simple, reliable solutions (like SQLite) over complex distributed systems.

---

## License

MIT © 2026
