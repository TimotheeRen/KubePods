use sqlx::{Pool, Postgres, Row, query, query_as};

use crate::{
    domains::provisioning::Desktop,
    errors::{metrics::MetricsError, provisioning::ProvisioningError},
};

pub trait PostgresRepositoryInterface {
    async fn add_desktop(
        &self,
        desktop: &Desktop,
        username: String,
    ) -> Result<(), ProvisioningError>;
    async fn get_desktops(&self, username: String) -> Result<Vec<Desktop>, ProvisioningError>;
    async fn remove_desktop(&self, name: String, username: String)
    -> Result<(), ProvisioningError>;
    async fn get_remaining_desktops(&self, username: String) -> Result<u32, MetricsError>;
}

#[derive(Clone)]
pub struct PostgresRepository {
    pub pool: Pool<Postgres>,
}

impl PostgresRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl PostgresRepositoryInterface for PostgresRepository {
    async fn add_desktop(
        &self,
        desktop: &Desktop,
        username: String,
    ) -> Result<(), ProvisioningError> {
        let id = format!("{}-{}", username, desktop.name);
        match query( "INSERT INTO desktops (id, name, username, distribution, desktop_environment) VALUES ($1, $2, $3, $4, $5)")
        .bind(id)
        .bind(desktop.name.clone())
        .bind(username)
        .bind(desktop.distribution.clone())
        .bind(desktop.desktop_environment.clone())
        .execute(&self.pool)
        .await
        {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(err)) => {
                if err.constraint() == Some("desktops_pkey") {
                    Err(ProvisioningError::DesktopAlreadyExist)
                } else {
                    Err(ProvisioningError::InternalServerError)
                }
            }
            Err(_) => Err(ProvisioningError::InternalServerError),
        }
    }

    async fn get_desktops(&self, username: String) -> Result<Vec<Desktop>, ProvisioningError> {
        let desktops = query_as::<_, Desktop>(
            "SELECT name, distribution, desktop_environment FROM desktops WHERE username = $1",
        )
        .bind(&username)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ProvisioningError::NoDesktopFound,
            _ => ProvisioningError::InternalServerError,
        })?;
        Ok(desktops)
    }

    async fn remove_desktop(
        &self,
        name: String,
        username: String,
    ) -> Result<(), ProvisioningError> {
        let id = format!("{}-{}", username, name);
        match query("DELETE FROM desktops WHERE id = $1")
            .bind(&id)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(ProvisioningError::InternalServerError),
        }
    }

    async fn get_remaining_desktops(&self, username: String) -> Result<u32, MetricsError> {
        let row = query("SELECT COUNT(*) FROM desktops WHERE username = $1")
            .bind(&username)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| MetricsError::InternalServerError)?;

        let count: i64 = row.get(0);
        Ok(count as u32)
    }
}
