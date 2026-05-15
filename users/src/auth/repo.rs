use sqlx::{Pool, Postgres, query};

use crate::auth::{error::CreateUserError, model::User};

pub trait AuthRepository {
    async fn create_user(&self, user: &User) -> Result<(), CreateUserError>;
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
        match query("INSERT INTO users (username, email, password) VALUES ($1, $2, $3)")
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password)
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
}
