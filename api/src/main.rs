mod users;
use crate::users::user::{
    login_service_client::LoginServiceClient, register_service_client::RegisterServiceClient,
};
use axum::Router;
use tonic::transport::Channel;

#[derive(Clone)]
struct AppState {
    pub user_register_client: RegisterServiceClient<Channel>,
    pub user_login_client: LoginServiceClient<Channel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let users_host = match std::env::var("USERS_HOST") {
        Ok(val) => val,
        Err(_) => "http://0.0.0.0:50051".to_string(),
    };

    let channel = Channel::from_shared(users_host)?.connect().await?;
    let user_register_client = RegisterServiceClient::new(channel.clone());
    let user_login_client = LoginServiceClient::new(channel);

    let state = AppState {
        user_register_client,
        user_login_client,
    };

    let app = Router::new()
        .nest("/users", users::routes::auth())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
