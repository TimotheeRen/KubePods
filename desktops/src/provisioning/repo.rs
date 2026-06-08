use desktops::DesktopSpec;
use kube::{
    Api, Client,
    api::{ObjectMeta, PostParams},
};
use sqlx::{Pool, Postgres};

use crate::provisioning::{error::ProvisioningError, model::Desktop};

pub trait KubernetesProvisiningRepository {
    async fn create_desktop(&self, desktop: &Desktop) -> Result<(), ProvisioningError>;
}

pub trait PostgresProvioningRepository {
    async fn add_desktop(&self, desktop: &Desktop) -> Result<(), ProvisioningError>;
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
    async fn create_desktop(&self, desktop: &Desktop) -> Result<(), ProvisioningError> {
        let desktop = desktops::Desktop {
            metadata: ObjectMeta {
                name: Some(desktop.name.clone()),
                namespace: Some("default".to_string()),
                ..Default::default()
            },
            spec: DesktopSpec {
                name: desktop.name.clone(),
                id: "id".to_string(),
                distribtion: desktop.distribution.clone(),
                desktop_environment: desktop.desktop_environement.clone(),
                max_ram: "1Gi".to_string(),
                max_cpu: "100m".to_string(),
            },
        };

        let desktops: Api<desktops::Desktop> = Api::default_namespaced(self.client.clone());
        match desktops.create(&PostParams::default(), &desktop).await {
            Ok(_) => Ok(()),
            Err(_) => Err(ProvisioningError::InternalServerError),
        }
    }
}

impl PostgresProvioningRepository for PostgresRepository {
    async fn add_desktop(&self, desktop: &Desktop) -> Result<(), ProvisioningError> {
        Ok(())
    }
}
