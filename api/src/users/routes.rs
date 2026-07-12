use axum::{
    Router,
    routing::{get, post},
};

use crate::{AppState, users::handlers};

pub fn auth() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::register_handler))
        .route("/login", post(handlers::login_handler))
        .route("/ping", get(handlers::ping))
        .route("/get_remaining_time", get(handlers::get_remaining_time))
        .route("/get_user_account", get(handlers::get_user_account))
}
