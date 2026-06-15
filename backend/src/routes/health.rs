use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;
use std::time::Instant;

use crate::{db::check_health, AppState};

static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

pub fn init_start_time() {
    START_TIME.get_or_init(Instant::now);
}

pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    let db_ok = check_health(&state.db).await;
    let uptime_secs = START_TIME
        .get()
        .map(|t| t.elapsed().as_secs())
        .unwrap_or(0);

    let status = if db_ok { "healthy" } else { "degraded" };

    Json(json!({
        "status": status,
        "db_ok": db_ok,
        "version": state.config.app_version,
        "uptime_secs": uptime_secs
    }))
}
