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
    let res = state
        .provisioning_auth_client
        .create_desktop(CreateDesktopRequest {
            name: desktop.name,
            distribution: desktop.distribution,
            desktop_environment: desktop.desktop_environment,
        })
        .await;
    println!("{:?}", res);
    Ok("Ok".to_string())
}
