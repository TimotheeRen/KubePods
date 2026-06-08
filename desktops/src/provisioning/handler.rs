use tonic::{Response, Status};

use crate::provisioning::repo::{KubernetesRepository, PostgresRepository};
use crate::provisioning::{
    handler::user::{
        CreateDesktopRequest, CreateDesktopResponse,
        provisioning_service_server::ProvisioningService,
    },
    service::ProvisioningServiceLayer,
};

pub mod user {
    tonic::include_proto!("provisioning");
}

pub struct ProvisioningImpl {
    pub service: ProvisioningServiceLayer<KubernetesRepository, PostgresRepository>,
}

#[tonic::async_trait]
impl ProvisioningService for ProvisioningImpl {
    async fn create_desktop(
        &self,
        request: tonic::Request<CreateDesktopRequest>,
    ) -> Result<tonic::Response<CreateDesktopResponse>, Status> {
        println!("Received a desktop creation request!");
        let req = request.into_inner();
        self.service
            .create_desktop(req.name, req.distribution, req.desktop_environment)
            .await;
        Ok(Response::new(CreateDesktopResponse {}))
    }
}
