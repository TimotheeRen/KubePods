use axum::{Json, extract::State, http::StatusCode};

use crate::{
    AppState,
    claims::Claims,
    desktops::{
        desktops_metrics::GetRemainingDesktopsRequest,
        desktops_provisioning::{CreateDesktopRequest, DeleteDesktopRequest, GetDesktopsRequest},
        schemas::{self, DesktopItem, GetRemainingDesktops},
    },
};

pub async fn create_desktop(
    State(mut state): State<AppState>,
    user: Claims,
    Json(desktop): Json<schemas::CreateDesktop>,
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

pub async fn delete_desktop(
    State(mut state): State<AppState>,
    user: Claims,
    Json(desktop): Json<schemas::DeleteDesktop>,
) -> Result<String, StatusCode> {
    state
        .provisioning_auth_client
        .delete_desktop(DeleteDesktopRequest {
            name: desktop.name,
            username: user.sub,
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok("".to_string())
}

pub async fn get_remaining_desktops(
    user: Claims,
    State(mut state): State<AppState>,
) -> Result<Json<GetRemainingDesktops>, StatusCode> {
    let res = state
        .desktops_metrics_client
        .get_remaining_desktops(GetRemainingDesktopsRequest { username: user.sub })
        .await
        .map_err(|e| {
            println!("{e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let inner_data = res.into_inner();
    let remaining = match user.role.as_str() {
        "starter" => 3,
        "standard" => 5,
        "premium" => 7,
        _ => 3,
    };

    Ok(Json(GetRemainingDesktops {
        created: inner_data.created as u8,
        remaining,
    }))
}
