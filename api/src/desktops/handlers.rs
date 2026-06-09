use axum::{Json, extract::State, http::StatusCode};

use crate::{
    AppState, Claims,
    desktops::{schemas, user::CreateDesktopRequest},
};

pub async fn create_desktop(
    State(mut state): State<AppState>,
    user: Claims,
    Json(desktop): Json<schemas::createDesktop>,
) -> Result<String, StatusCode> {
    state
        .provisioning_auth_client
        .create_desktop(CreateDesktopRequest {
            name: desktop.name,
            distribution: desktop.distribution,
            desktop_environment: desktop.desktop_environment,
            username: user.sub,
        })
        .await
        .map_err(|e| match e.code() {
            tonic::Code::AlreadyExists => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;
    Ok("".to_string())
}
