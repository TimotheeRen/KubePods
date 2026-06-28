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
}
