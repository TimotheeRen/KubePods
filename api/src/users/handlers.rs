use crate::AppState;
use crate::users::user::LoginRequest;
use crate::users::{
    schemas::{self},
    user::RegisterRequest,
};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn register_handler(
    State(mut state): State<AppState>,
    Json(user): Json<schemas::RegisterUser>,
) -> Result<String, StatusCode> {
    println!("Received a register request from user: {}", user.username);
    state
        .user_auth_client
        .register(RegisterRequest {
            email: user.email,
            username: user.username,
            password: user.password,
        })
        .await
        .map_err(|e| {
            println!("Grpc error: {}", e);
            match e.code() {
                tonic::Code::AlreadyExists => StatusCode::CONFLICT,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        })?;
    Ok("Registered.".to_string())
}

pub async fn login_handler(
    State(mut state): State<AppState>,
    Json(user): Json<schemas::LoginUser>,
) -> Result<String, StatusCode> {
    println!("Received a login request from user: {}", user.username);
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
