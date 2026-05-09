mod users;
use crate::users::user::register_service_client::RegisterServiceClient;
use axum::{Router, routing::post};

#[derive(Clone)]
struct AppState {
    pub user_client: RegisterServiceClient<tonic::transport::Channel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_client = RegisterServiceClient::connect("http://[::1]:50051").await?;

    let state = AppState {
        user_client: user_client,
    };

    let app = Router::new()
        .nest("/users", users::routes::auth())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
