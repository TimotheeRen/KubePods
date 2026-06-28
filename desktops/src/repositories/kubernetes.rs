use desktops::DesktopSpec;
use kube::{
    Api, Client,
    api::{DeleteParams, ObjectMeta, PostParams},
};

use crate::domains::{error::ProvisioningError, provisioning::Desktop};

pub trait KubernetesProvisiningRepository {
    async fn create_desktop(
        &self,
        desktop: &Desktop,
        username: String,
    ) -> Result<(), ProvisioningError>;

    async fn delete_desktop(&self, name: String, username: String)
    -> Result<(), ProvisioningError>;
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

    async fn delete_desktop(
        &self,
        name: String,
        username: String,
    ) -> Result<(), ProvisioningError> {
        let id = format!("{}-{}", username.to_lowercase(), name);
        let desktops: Api<desktops::Desktop> = Api::namespaced(self.client.clone(), "desktops");
        match desktops.delete(&id, &DeleteParams::default()).await {
            Ok(_) => Ok(()),
            Err(_) => Err(ProvisioningError::InternalServerError),
        }
    }
}
