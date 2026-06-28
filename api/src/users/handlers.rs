use crate::users::schemas::RemainingTime;
use crate::users::user_auth::LoginRequest;
use crate::users::user_info::RemainingTimeRequest;
use crate::users::{
    schemas::{self},
    user_auth::RegisterRequest,
};
use crate::{AppState, claims::Claims};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn register_handler(
    State(mut state): State<AppState>,
    Json(user): Json<schemas::RegisterUser>,
) -> Result<String, StatusCode> {
    let res = state
        .user_auth_client
        .register(RegisterRequest {
            email: user.email,
            username: user.username,
            password: user.password,
        })
        .await
        .map_err(|e| match e.code() {
            tonic::Code::AlreadyExists => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;
    Ok(res.into_inner().token)
}

pub async fn login_handler(
    State(mut state): State<AppState>,
    Json(user): Json<schemas::LoginUser>,
) -> Result<String, StatusCode> {
    let res = state
        .user_auth_client
        .login(LoginRequest {
            username: user.username,
            password: user.password,
        })
        .await
        .map_err(|e| match e.code() {
            tonic::Code::Unauthenticated => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;
    Ok(res.into_inner().token)
}

pub async fn ping(_: Claims) -> String {
    "Pong!".to_string()
}

pub async fn get_remaining_time(
    user: Claims,
    State(mut state): State<AppState>,
) -> Result<Json<RemainingTime>, StatusCode> {
    let res = state
        .user_info_client
        .remaining_time(RemainingTimeRequest { username: user.sub })
        .await
        .map_err(|e| {
            println!("{e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let inner_data = res.into_inner();
    let remaining = match user.role.as_str() {
        "starter" => 100,
        "standard" => 200,
        "premium" => 300,
        _ => 100,
    };

    Ok(Json(RemainingTime {
        utilization: inner_data.utilization as u8,
        remaining,
    }))
}
