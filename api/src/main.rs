mod desktops;
mod users;

use crate::{
    desktops::user::provisioning_service_client::ProvisioningServiceClient,
    users::user::auth_service_client::AuthServiceClient,
};
use axum::{
    Router,
    extract::FromRequestParts,
    http::{HeaderValue, StatusCode, request::Parts},
    response::{IntoResponse, Response},
    routing::get,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    pub user_auth_client: AuthServiceClient<Channel>,
    pub provisioning_auth_client: ProvisioningServiceClient<Channel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: i64,
}

pub enum AuthError {
    MissingToken,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = StatusCode::UNAUTHORIZED;
        let message = match self {
            AuthError::MissingToken => "Missing token",
            AuthError::InvalidToken => "Invalid token",
        };
        (status, message).into_response()
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AuthError::MissingToken)?;
        let key = &DecodingKey::from_secret("secret".as_ref()); // TODO: add secret var
        let validation = &Validation::new(jsonwebtoken::Algorithm::HS256);
        let token = decode::<Claims>(bearer.token(), key, validation)
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(token.claims)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let users_host = match std::env::var("USERS_HOST") {
        Ok(val) => val,
        Err(_) => "http://0.0.0.0:50051".to_string(),
    };

    let desktops_host = match std::env::var("DESKTOPS_HOST") {
        Ok(val) => val,
        Err(_) => "http://0.0.0.0:50052".to_string(),
    };

    let users_channel = Channel::from_shared(users_host)?.connect().await?;
    let desktops_channel = Channel::from_shared(desktops_host)?.connect().await?;
    let user_auth_client = AuthServiceClient::new(users_channel);
    let provisioning_auth_client = ProvisioningServiceClient::new(desktops_channel);

    let state = AppState {
        user_auth_client,
        provisioning_auth_client,
    };

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(), // DEV
            "http://localhost:8081".parse::<HeaderValue>().unwrap(), // DEBUG PROD
            "http://localhost:8080".parse::<HeaderValue>().unwrap(), // PROD
        ])
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/ping", get(ping))
        .nest("/users", users::routes::auth())
        .nest("/desktops", desktops::routes::provisioning())
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

// Used for auth
pub async fn ping(_: Claims) -> String {
    "Pong!".to_string()
}
