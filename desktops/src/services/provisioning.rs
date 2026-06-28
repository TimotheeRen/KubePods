use crate::{
    domains::provisioning::Desktop,
    errors::provisioning::ProvisioningError,
    repositories::{
        kubernetes::KubernetesRepositoryInterface, postgres::PostgresRepositoryInterface,
    },
};

pub struct ProvisioningServiceLayer<
    K: KubernetesRepositoryInterface,
    P: PostgresRepositoryInterface,
> {
    kubernetes_repo: K,
    postgres_repo: P,
}

impl<K: KubernetesRepositoryInterface, P: PostgresRepositoryInterface>
    ProvisioningServiceLayer<K, P>
{
    pub fn new(kubernetes_repo: K, postgres_repo: P) -> Self {
        Self {
            kubernetes_repo,
            postgres_repo,
        }
    }

    pub async fn create_desktop(
        &self,
        name: String,
        distribution: String,
        desktop_environment: String,
        username: String,
    ) -> Result<(), ProvisioningError> {
        let desktop = Desktop {
            name,
            distribution,
            desktop_environment,
        };
        self.kubernetes_repo
            .create_desktop(&desktop, username.clone())
            .await?;
        self.postgres_repo.add_desktop(&desktop, username).await?;
        Ok(())
    }

    pub async fn get_desktops(&self, username: String) -> Result<Vec<Desktop>, ProvisioningError> {
        self.postgres_repo.get_desktops(username).await
    }

    pub async fn delete_desktop(
        &self,
        name: String,
        username: String,
    ) -> Result<(), ProvisioningError> {
        self.kubernetes_repo
            .delete_desktop(name.clone(), username.clone())
            .await?;
        self.postgres_repo.remove_desktop(name, username).await?;
        Ok(())
    }
}
