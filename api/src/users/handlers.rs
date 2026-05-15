use crate::AppState;
use crate::users::schemas::{self, RegisterUser};
use crate::users::user::RegisterRequest;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn register_handler(
    State(mut state): State<AppState>,
    Json(user): Json<schemas::RegisterUser>,
) -> Result<String, StatusCode> {
    println!("Received a register request from user: {}", user.username);
    state
        .user_client
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
