use std::time::Duration;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::{Pool, Postgres, Row, query};

use crate::auth::{
    error::{CheckPasswordError, CreateUserError},
    model::{LoginUser, User, UserClaims},
};

pub trait AuthRepository {
    async fn create_user(&self, user: &User) -> Result<(), CreateUserError>;
    async fn check_password(&self, user: LoginUser) -> Result<String, CheckPasswordError>;
    async fn generate_token(
        &self,
        username: String,
        password: String,
        hashed_password: &str,
    ) -> Result<String, CheckPasswordError>; // TODO: Add
    // another error type
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
    async fn create_user(&self, user: &User) -> Result<(), CreateUserError> {
        let password = Argon2::default()
            .hash_password(user.password.as_bytes())
            .map_err(|_| CreateUserError::HashPasswordError)?
            .to_string();

        match query("INSERT INTO users (username, email, password) VALUES ($1, $2, $3)")
            .bind(&user.username)
            .bind(&user.email)
            .bind(&password)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(err)) => {
                println!("{:?}", err.constraint());
                if err.constraint() == Some("users_pkey") {
                    Err(CreateUserError::UserAlreadyExists)
                } else {
                    Err(CreateUserError::DatabaseError)
                }
            }
            Err(_) => Err(CreateUserError::DatabaseError),
        }
    }

    async fn check_password(&self, user: LoginUser) -> Result<String, CheckPasswordError> {
        let res = query("SELECT password FROM users WHERE username = $1")
            .bind(&user.username)
            .fetch_optional(&self.pool)
            .await;
        match res {
            Ok(Some(row)) => Ok(row.get("password")),
            Ok(None) => Err(CheckPasswordError::WrongPassword),
            Err(_) => Err(CheckPasswordError::DatabaseError),
        }
    }

    async fn generate_token(
        &self,
        username: String,
        password: String,
        hashed_password: &str,
    ) -> Result<String, CheckPasswordError> {
        let hash = PasswordHash::new(hashed_password).map_err(|e| {
            println!("PasswordHash error: {:?}", e);
            CheckPasswordError::DatabaseError
        })?;

        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .map_err(|_| CheckPasswordError::WrongPassword)?;

        println!("Correct credentials !");

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
        .map_err(|_| CheckPasswordError::DatabaseError)?;

        Ok(token)
    }
}
