use crate::users::schemas;
use axum::Json;

pub async fn register_handler(Json(payload): Json<schemas::RegisterUser>) -> String {
    println!(
        "Received a register request from user: {}",
        payload.username
    );
    return "Registered.".to_string();
}
