use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{AppState, desktops::handlers};

pub fn provisioning() -> Router<AppState> {
    Router::new()
        .route("/create_desktop", post(handlers::create_desktop))
        .route("/get_desktops", get(handlers::get_desktops))
        .route("/delete_desktop", delete(handlers::delete_desktop))
}
