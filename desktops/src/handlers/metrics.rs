use tonic::{Response, Status};

use crate::{
    handlers::metrics::desktops_metrics::{
        GetRemainingDesktopsRequest, GetRemainingDesktopsResponse,
        metrics_service_server::MetricsService,
    },
    repositories::{kubernetes::KubernetesRepository, postgres::PostgresRepository},
    services::metrics::MetricsServiceLayer,
};

pub mod desktops_metrics {
    tonic::include_proto!("metrics");
}

pub struct MetricsHandler {
    pub service: MetricsServiceLayer<KubernetesRepository, PostgresRepository>,
}

#[tonic::async_trait]
impl MetricsService for MetricsHandler {
    async fn get_remaining_desktops(
        &self,
        request: tonic::Request<GetRemainingDesktopsRequest>,
    ) -> Result<tonic::Response<GetRemainingDesktopsResponse>, Status> {
        let req = request.into_inner();
        let created = self
            .service
            .get_remaining_desktops(req.username)
            .await
            .map_err(|_| Status::internal("Internal server error"))?;
        Ok(Response::new(GetRemainingDesktopsResponse { created }))
    }
}
