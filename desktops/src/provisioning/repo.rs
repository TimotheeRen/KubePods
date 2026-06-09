use desktops::DesktopSpec;
use kube::{
    Api, Client,
    api::{ObjectMeta, PostParams},
};
use sha2::{Digest, Sha256};
use sqlx::{Pool, Postgres};

use crate::provisioning::{error::ProvisioningError, model::Desktop};

pub trait KubernetesProvisiningRepository {
    async fn create_desktop(
        &self,
        desktop: &Desktop,
        username: String,
    ) -> Result<(), ProvisioningError>;
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
    async fn create_desktop(
        &self,
        desktop: &Desktop,
        username: String,
    ) -> Result<(), ProvisioningError> {
        let id = format!("{}-{}", username.to_lowercase(), desktop.name.clone());
        /*let hash = Sha256::digest(base.as_bytes());
        let hex = hex::encode(hash);
        let id = format!("d-{}", &hex[..61]);*/

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
                desktop_environment: desktop.desktop_environement.clone(),
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
    async fn add_desktop(&self, desktop: &Desktop) -> Result<(), ProvisioningError> {
        Ok(())
    }
}
