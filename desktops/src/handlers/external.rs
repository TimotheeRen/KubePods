use tonic::{Response, Status};

use crate::{
    handlers::external::desktops_external::{
        ChangeDesktopsUserRequest, ChangeDesktopsUserResponse,
        desktops_external_service_server::DesktopsExternalService,
    },
    repositories::{kubernetes::KubernetesRepository, postgres::PostgresRepository},
    services::external::ExternalServiceLayer,
};

pub mod desktops_external {
    tonic::include_proto!("desktops_external");
}

pub struct ExternalHandler {
    pub service: ExternalServiceLayer<KubernetesRepository, PostgresRepository>,
}

#[tonic::async_trait]
impl DesktopsExternalService for ExternalHandler {
    async fn change_desktops_user(
        &self,
        request: tonic::Request<ChangeDesktopsUserRequest>,
    ) -> Result<tonic::Response<ChangeDesktopsUserResponse>, Status> {
        let req = request.into_inner();
        self.service
            .change_desktops_user(req.username, req.old_username)
            .await
            .map_err(|_| Status::internal("Internal server error"))?;
        Ok(Response::new(ChangeDesktopsUserResponse {}))
    }
}
