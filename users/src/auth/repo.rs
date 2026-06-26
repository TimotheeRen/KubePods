use std::time::Duration;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::{Pool, Postgres, Row, query};

use crate::auth::{
    error::AuthError,
    handler::user::UsersTicks,
    model::{LoginUser, User, UserClaims},
};

pub trait AuthRepository {
    async fn create_user(&self, user: &User) -> Result<(), AuthError>;
    async fn check_password(&self, user: LoginUser) -> Result<String, AuthError>;
    async fn generate_token(
        &self,
        username: String,
        password: String,
        hashed_password: &str,
    ) -> Result<String, AuthError>;
    async fn increment_ticks(&self, users_ticks: Vec<UsersTicks>) -> Result<(), AuthError>;
}

pub struct PostgresAuthRepository {
    pub pool: Pool<Postgres>,
}

impl PostgresAuthRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl AuthRepository for PostgresAuthRepository {
    async fn create_user(&self, user: &User) -> Result<(), AuthError> {
        let password = Argon2::default()
            .hash_password(user.password.as_bytes())
            .map_err(|_| AuthError::InternalServerError)?
            .to_string();

        match query("INSERT INTO users (username, email, password) VALUES ($1, $2, $3)")
            .bind(user.username.to_lowercase())
            .bind(&user.email)
            .bind(&password)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(err)) => {
                if err.constraint() == Some("users_pkey") {
                    Err(AuthError::UserAlreadyExists)
                } else {
                    Err(AuthError::InternalServerError)
                }
            }
            Err(_) => Err(AuthError::InternalServerError),
        }
    }

    async fn check_password(&self, user: LoginUser) -> Result<String, AuthError> {
        let res = query("SELECT password FROM users WHERE username = $1")
            .bind(&user.username)
            .fetch_optional(&self.pool)
            .await;
        match res {
            Ok(Some(row)) => Ok(row.get("password")),
            Ok(None) => Err(AuthError::WrongPassword),
            Err(_) => Err(AuthError::InternalServerError),
        }
    }

    async fn generate_token(
        &self,
        username: String,
        password: String,
        hashed_password: &str,
    ) -> Result<String, AuthError> {
        let hash = PasswordHash::new(hashed_password).map_err(|_| AuthError::WrongPassword)?;

        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .map_err(|_| AuthError::WrongPassword)?;

        let expiration = Utc::now() + Duration::from_mins(90);

        let claims = UserClaims {
            sub: username,
            role: "user".to_string(),
            exp: expiration.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()), // TODO: add secret var
        )
        .map_err(|_| AuthError::InternalServerError)?;

        Ok(token)
    }

    async fn increment_ticks(&self, users_ticks: Vec<UsersTicks>) -> Result<(), AuthError> {
        for tick in users_ticks {
            let _ = query("UPDATE users SET utilization = utilization + $1 WHERE username = $2")
                .bind(tick.tick as i32)
                .bind(&tick.username)
                .execute(&self.pool)
                .await
                .map_err(|_| AuthError::InternalServerError)?;
        }

        Ok(())
    }
}
