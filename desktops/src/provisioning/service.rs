use crate::provisioning::{
    error::ProvisioningError,
    model::Desktop,
    repo::{KubernetesProvisiningRepository, PostgresProvioningRepository},
};

pub struct ProvisioningServiceLayer<
    K: KubernetesProvisiningRepository,
    P: PostgresProvioningRepository,
> {
    kubernetes_repo: K,
    postgres_repo: P,
}

impl<K: KubernetesProvisiningRepository, P: PostgresProvioningRepository>
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
        desktop_environement: String,
    ) -> Result<(), ProvisioningError> {
        let desktop = Desktop {
            name,
            distribution,
            desktop_environement,
        };
        self.kubernetes_repo.create_desktop(&desktop).await;
        self.postgres_repo.add_desktop(&desktop).await;
        Ok(())
    }
}
