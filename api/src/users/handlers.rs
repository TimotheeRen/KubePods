use crate::users::{schemas, services};
use axum::Json;

pub async fn register_handler(Json(payload): Json<schemas::RegisterUser>) -> String {
    services::register_user(payload).await;
    return "Registered.".to_string();
}
