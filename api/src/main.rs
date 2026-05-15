mod users;
use crate::users::user::register_service_client::RegisterServiceClient;
use axum::{Router, routing::post};

#[derive(Clone)]
struct AppState {
    pub user_client: RegisterServiceClient<tonic::transport::Channel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let users_host = match std::env::var("USERS_HOST") {
        Ok(val) => val,
        Err(_) => "http://0.0.0.0:50051".to_string(),
    };
    let user_client = RegisterServiceClient::connect(users_host).await?;

    let state = AppState { user_client };

    let app = Router::new()
        .nest("/users", users::routes::auth())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
