use axum::{Json, extract::State, http::StatusCode};

use crate::{AppState, desktops::schemas};

pub async fn create_desktop(
    State(mut state): State<AppState>,
    Json(desktop): Json<schemas::createDesktop>,
) -> Result<String, StatusCode> {
    Ok("Ok".to_string())
}
