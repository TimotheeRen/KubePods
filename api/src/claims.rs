use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

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
        let jwt_secret = match std::env::var("JWT_SECRET") {
            Ok(val) => val,
            Err(_) => "jwt_default_secret".to_string(),
        };

        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AuthError::MissingToken)?;
        let key = &DecodingKey::from_secret(jwt_secret.as_ref());
        let validation = &Validation::new(jsonwebtoken::Algorithm::HS256);
        let token = decode::<Claims>(bearer.token(), key, validation)
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(token.claims)
    }
}
