use axum::{Router, routing::post};

use crate::{AppState, users::handlers};

pub fn auth() -> Router<AppState> {
    Router::new().route("/register", post(handlers::register_handler))
}
