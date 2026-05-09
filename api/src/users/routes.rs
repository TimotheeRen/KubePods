use axum::{Router, routing::post};

use crate::users::handlers;

pub fn auth() -> Router {
    Router::new().route("/register", post(handlers::register_handler))
}
