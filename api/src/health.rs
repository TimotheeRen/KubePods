use axum::{Router, http::StatusCode, routing::get};

use crate::AppState;

pub fn probes() -> Router<AppState> {
    Router::new().route("/live", get(liveness_probe))
}

pub async fn liveness_probe() -> StatusCode {
    StatusCode::OK
}
