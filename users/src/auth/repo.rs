use std::time::{Duration, SystemTime};

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, crypto::CryptoProvider, encode};
use sqlx::{Pool, Postgres, Row, query};

use crate::auth::{
    error::{CheckPasswordError, CreateUserError},
    model::{LoginUser, User, UserClaims},
};

pub trait AuthRepository {
    async fn create_user(&self, user: &User) -> Result<(), CreateUserError>;
    async fn check_password(&self, user: LoginUser) -> Result<String, CheckPasswordError>;
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
        let password: String = match res {
            Ok(Some(row)) => row.get("password"),
            Ok(None) => return Err(CheckPasswordError::WrongPassword),
            Err(_) => return Err(CheckPasswordError::DatabaseError),
        };

        let hash = PasswordHash::new(&password).map_err(|e| {
            println!("PasswordHash error: {:?}", e);
            CheckPasswordError::DatabaseError
        })?;

        Argon2::default()
            .verify_password(user.password.as_bytes(), &hash)
            .map_err(|_| CheckPasswordError::WrongPassword)?;

        println!("Correct credentials !");

        let expiration = Utc::now() + Duration::from_mins(90);

        let claims = UserClaims {
            sub: user.username,
            role: "user".to_string(),
            exp: expiration.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()), // TODO add secret var
        )
        .map_err(|_| CheckPasswordError::DatabaseError)?;

        Ok(token)
    }
}
