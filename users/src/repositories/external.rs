use std::time::Duration;

use desktops_external::desktops_external_service_client::DesktopsExternalServiceClient;
use tonic::transport::Channel;

use crate::{
    errors::info::InfoError, repositories::external::desktops_external::ChangeDesktopsUserRequest,
};

pub mod desktops_external {
    tonic::include_proto!("desktops_external");
}

pub trait ExternalRepositoryInterface {
    async fn update_settings(
        &self,
        username: String,
        old_username: String,
    ) -> Result<(), InfoError>;
}

#[derive(Clone)]
pub struct ExternalRepository {
    desktops_external_client: DesktopsExternalServiceClient<Channel>,
}

impl ExternalRepository {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let desktops_host = match std::env::var("DESKTOPS_HOST") {
            Ok(val) => val,
            Err(_) => "http://0.0.0.0:50052".to_string(),
        };

        let desktops_channel = Channel::from_shared(desktops_host)?
            .connect_timeout(Duration::from_secs(30))
            .timeout(Duration::from_secs(5))
            .connect()
            .await?;

        let desktops_external_client = DesktopsExternalServiceClient::new(desktops_channel);
        Ok(Self {
            desktops_external_client,
        })
    }
}

impl ExternalRepositoryInterface for ExternalRepository {
    async fn update_settings(
        &self,
        username: String,
        old_username: String,
    ) -> Result<(), InfoError> {
        let mut client = self.desktops_external_client.clone();
        client
            .change_desktops_user(ChangeDesktopsUserRequest {
                username,
                old_username,
            })
            .await
            .map_err(|_| InfoError::InternalServerError)?;
        Ok(())
    }
}
