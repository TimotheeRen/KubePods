use desktops::DesktopSpec;
use kube::{
    Api, Client,
    api::{ObjectMeta, PostParams},
};
use sqlx::{Pool, Postgres, query, query_as};

use crate::provisioning::{error::ProvisioningError, model::Desktop};

pub trait KubernetesProvisiningRepository {
    async fn create_desktop(
        &self,
        desktop: &Desktop,
        username: String,
    ) -> Result<(), ProvisioningError>;
}

pub trait PostgresProvioningRepository {
    async fn add_desktop(
        &self,
        desktop: &Desktop,
        username: String,
    ) -> Result<(), ProvisioningError>;
    async fn get_desktops(&self, username: String) -> Result<Vec<Desktop>, ProvisioningError>;
}

pub struct PostgresRepository {
    pub pool: Pool<Postgres>,
}

impl PostgresRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

pub struct KubernetesRepository {
    pub client: Client,
}

impl KubernetesRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl KubernetesProvisiningRepository for KubernetesRepository {
    async fn create_desktop(
        &self,
        desktop: &Desktop,
        username: String,
    ) -> Result<(), ProvisioningError> {
        let id = format!("{}-{}", username.to_lowercase(), desktop.name.clone());

        let desktop = desktops::Desktop {
            metadata: ObjectMeta {
                name: Some(id.clone()),
                namespace: Some("desktops".to_string()),
                ..Default::default()
            },
            spec: DesktopSpec {
                name: desktop.name.clone(),
                id,
                distribtion: desktop.distribution.clone(),
                desktop_environment: desktop.desktop_environment.clone(),
                max_ram: "1Gi".to_string(),
                max_cpu: "100m".to_string(),
            },
        };

        let desktops: Api<desktops::Desktop> = Api::namespaced(self.client.clone(), "desktops");
        match desktops.create(&PostParams::default(), &desktop).await {
            Ok(_) => Ok(()),
            Err(kube::Error::Api(e)) if e.code == 409 => {
                Err(ProvisioningError::DesktopAlreadyExist)
            }
            Err(_) => Err(ProvisioningError::InternalServerError),
        }
    }
}

impl PostgresProvioningRepository for PostgresRepository {
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
}
