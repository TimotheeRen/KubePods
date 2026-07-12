use tonic::{Response, Status};
use user_info::info_service_server::InfoService;

use crate::{repositories::postgres::PostgresRepository, services::info::InfoServiceLayer};

pub mod user_info {
    tonic::include_proto!("info");
}

pub struct InfoImpl {
    pub service: InfoServiceLayer<PostgresRepository>,
}

#[tonic::async_trait]
impl InfoService for InfoImpl {
    async fn remaining_time(
        &self,
        request: tonic::Request<user_info::RemainingTimeRequest>,
    ) -> std::result::Result<tonic::Response<user_info::RemainingTimeResponse>, tonic::Status> {
        let req = request.into_inner();
        let usage = self
            .service
            .get_remaining_time(req.username)
            .await
            .map_err(|_| Status::internal("Internal server error"))?;

        Ok(Response::new(user_info::RemainingTimeResponse {
            utilization: usage,
        }))
    }

    async fn get_user_account(
        &self,
        request: tonic::Request<user_info::GetUserAccountRequest>,
    ) -> std::result::Result<tonic::Response<user_info::GetUserAccountResponse>, tonic::Status>
    {
        let req = request.into_inner();
        let account = self
            .service
            .get_account(req.username)
            .await
            .map_err(|_| Status::internal("Internal server error"))?;

        Ok(Response::new(user_info::GetUserAccountResponse {
            email: account.email,
            username: account.username,
        }))
    }

    async fn save_settings(
        &self,
        request: tonic::Request<user_info::SaveSettingsRequest>,
    ) -> std::result::Result<tonic::Response<user_info::SaveSettingsResponse>, tonic::Status> {
        let req = request.into_inner();
        self.service
            .save_settings(req.email, req.username, req.old_username)
            .await
            .map_err(|_| Status::internal("Internal server error"))?;

        Ok(Response::new(user_info::SaveSettingsResponse {}))
    }
}
