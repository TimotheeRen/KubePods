use sqlx::{Pool, Postgres, query};

use crate::auth::model::User;

pub trait AuthRepository {
    async fn create_user(&self, user: &User) -> Result<(), sqlx::Error>;
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
    async fn create_user(&self, user: &User) -> Result<(), sqlx::Error> {
        query("INSERT INTO users (username, email, password) VALUES ($1, $2, $3)")
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password)
            .execute(&self.pool)
            .await;

        Ok(())
    }
}
