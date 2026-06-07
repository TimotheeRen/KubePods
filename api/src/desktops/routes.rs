use axum::{Router, routing::post};

use crate::{AppState, desktops::handlers};

pub fn provisioning() -> Router<AppState> {
    Router::new().route("/create_desktop", post(handlers::create_desktop))
}
