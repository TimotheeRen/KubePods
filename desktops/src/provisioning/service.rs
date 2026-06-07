use crate::provisioning::{error::ProvisioningError, model::Desktop, repo::ProvisioningRepository};

pub struct ProvisioningServiceLayer<R: ProvisioningRepository> {
    repo: R,
}

impl<R: ProvisioningRepository> ProvisioningServiceLayer<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
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
        self.repo.create_desktop(&desktop).await
    }
}
