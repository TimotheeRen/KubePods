mod users;
use crate::users::user::auth_service_client::AuthServiceClient;
use axum::Router;
use tonic::transport::Channel;

#[derive(Clone)]
struct AppState {
    pub user_auth_client: AuthServiceClient<Channel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let users_host = match std::env::var("USERS_HOST") {
        Ok(val) => val,
        Err(_) => "http://0.0.0.0:50051".to_string(),
    };

    let channel = Channel::from_shared(users_host)?.connect().await?;
    let user_auth_client = AuthServiceClient::new(channel);

    let state = AppState { user_auth_client };

    let app = Router::new()
        .nest("/users", users::routes::auth())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
