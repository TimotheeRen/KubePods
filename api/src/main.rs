mod users;
use crate::users::user::auth_service_client::AuthServiceClient;
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

    let channel = Channel::from_shared(users_host)?.connect().await?;
    let user_auth_client = AuthServiceClient::new(channel);

    let state = AppState { user_auth_client };

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "http://localhost:8080".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/ping", get(ping))
        .nest("/users", users::routes::auth())
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
