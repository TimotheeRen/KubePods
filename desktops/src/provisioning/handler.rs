use tonic::{Response, Status};

use crate::provisioning::error::ProvisioningError;
use crate::provisioning::handler::user::{
    DeleteDesktopRequest, DeleteDesktopResponse, GetDesktopsRequest, GetDesktopsResponse,
};
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
        let req = request.into_inner();
        self.service
            .create_desktop(
                req.name,
                req.distribution,
                req.desktop_environment,
                req.username,
            )
            .await
            .map_err(|e| match e {
                ProvisioningError::DesktopAlreadyExist => {
                    Status::already_exists("Desktop already exist")
                }
                _ => Status::internal("Internal server error"),
            })?;
        Ok(Response::new(CreateDesktopResponse {}))
    }

    async fn get_desktops(
        &self,
        request: tonic::Request<GetDesktopsRequest>,
    ) -> Result<tonic::Response<GetDesktopsResponse>, Status> {
        let req = request.into_inner();
        let res = self
            .service
            .get_desktops(req.username)
            .await
            .map_err(|e| match e {
                ProvisioningError::NoDesktopFound => Status::not_found("No desktop found."),
                _ => Status::internal("Internal server error"),
            })?;
        let desktops = res
            .into_iter()
            .map(|d| user::Desktop {
                name: d.name,
                distribution: d.distribution,
                desktop_environment: d.desktop_environment,
            })
            .collect();
        Ok(Response::new(GetDesktopsResponse { desktops }))
    }

    async fn delete_desktop(
        &self,
        request: tonic::Request<DeleteDesktopRequest>,
    ) -> Result<tonic::Response<DeleteDesktopResponse>, Status> {
        let req = request.into_inner();
        self.service
            .delete_desktop(req.name, req.username)
            .await
            .map_err(|_| Status::internal("Internal server error"))?;
        Ok(Response::new(DeleteDesktopResponse {}))
    }
}
