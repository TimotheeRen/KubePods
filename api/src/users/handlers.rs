use crate::users::schemas::RemainingTime;
use crate::users::user::LoginRequest;
use crate::users::{
    schemas::{self},
    user::RegisterRequest,
};
use crate::{AppState, Claims};
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

pub async fn get_remaining_time(
    user: Claims,
    State(mut state): State<AppState>,
) -> Result<Json<RemainingTime>, StatusCode> {
    Ok(Json(RemainingTime {
        utilization: 8,
        remaining: 100,
    }))
}

pub async fn ping(_: Claims) -> String {
    "Pong!".to_string()
}
