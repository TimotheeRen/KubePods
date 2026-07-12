use crate::{
    handlers::external::user_external::user_external_service_server::UserExternalService,
    repositories::postgres::PostgresRepository, services::external::ExternalServiceLayer,
};
use tonic::{Response, Status};

pub mod user_external {
    tonic::include_proto!("user_external");
}

pub struct ExternalImpl {
    pub service: ExternalServiceLayer<PostgresRepository>,
}

#[tonic::async_trait]
impl UserExternalService for ExternalImpl {
    async fn increment_chronometer(
        &self,
        request: tonic::Request<user_external::IncrementChronometerRequest>,
    ) -> std::result::Result<
        tonic::Response<user_external::IncrementChronometerResponse>,
        tonic::Status,
    > {
        let req = request.into_inner();
        self.service
            .increment_chronometer(req.users_ticks)
            .await
            .map_err(|_| Status::internal("Internal server error"))?;

        Ok(Response::new(
            user_external::IncrementChronometerResponse {},
        ))
    }
}
