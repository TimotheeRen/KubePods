use tonic::Response;
use user_info::info_service_server::InfoService;

pub mod user_info {
    tonic::include_proto!("info");
}

pub struct InfoImpl {}

#[tonic::async_trait]
impl InfoService for InfoImpl {
    async fn remaining_time(
        &self,
        request: tonic::Request<user_info::RemainingTimeRequest>,
    ) -> std::result::Result<tonic::Response<user_info::RemainingTimeResponse>, tonic::Status> {
        let req = request.into_inner();
        // self.service.increment_chronometer(req.username).await;

        Ok(Response::new(user_info::RemainingTimeResponse {
            utilization: 6,
            remaining: 100,
        }))
    }
}
