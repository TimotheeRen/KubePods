use axum::{Json, extract::State, http::StatusCode};

use crate::{
    AppState, Claims,
    desktops::{
        schemas::{self, DesktopItem},
        user::{CreateDesktopRequest, GetDesktopsRequest},
    },
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

pub async fn get_desktops(
    State(mut state): State<AppState>,
    user: Claims,
) -> Result<Json<Vec<DesktopItem>>, StatusCode> {
    let res = state
        .provisioning_auth_client
        .get_desktops(GetDesktopsRequest { username: user.sub })
        .await
        .map_err(|e| match e.code() {
            tonic::Code::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .into_inner();

    let desktops: Vec<DesktopItem> = res
        .desktops
        .into_iter()
        .map(|d| DesktopItem {
            name: d.name,
            distribution: d.distribution,
            desktop_environment: d.desktop_environment,
        })
        .collect();

    Ok(Json(desktops))
}
