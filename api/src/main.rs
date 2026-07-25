mod claims;
mod desktops;
mod health;
mod middlewares;
mod users;

use crate::{
    desktops::{
        desktops_metrics::metrics_service_client::MetricsServiceClient,
        desktops_provisioning::provisioning_service_client::ProvisioningServiceClient,
    },
    middlewares::metrics,
    users::{
        user_auth::auth_service_client::AuthServiceClient,
        user_info::info_service_client::InfoServiceClient,
    },
};
use axum::{Router, http::HeaderValue, middleware::from_fn};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use tonic::transport::Channel;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    pub provisioning_auth_client: ProvisioningServiceClient<Channel>,
    pub desktops_metrics_client: MetricsServiceClient<Channel>,
    pub user_auth_client: AuthServiceClient<Channel>,
    pub user_info_client: InfoServiceClient<Channel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let users_host = match std::env::var("USERS_HOST") {
        Ok(val) => val,
        Err(_) => "http://0.0.0.0:50051".to_string(),
    };

    let desktops_host = match std::env::var("DESKTOPS_HOST") {
        Ok(val) => val,
        Err(_) => "http://0.0.0.0:50052".to_string(),
    };

    let users_channel = Channel::from_shared(users_host)?.connect().await?;
    let desktops_channel = Channel::from_shared(desktops_host)?.connect().await?;
    let user_auth_client = AuthServiceClient::new(users_channel.clone());
    let user_info_client = InfoServiceClient::new(users_channel);
    let provisioning_auth_client = ProvisioningServiceClient::new(desktops_channel.clone());
    let desktops_metrics_client = MetricsServiceClient::new(desktops_channel);

    let state = AppState {
        user_auth_client,
        desktops_metrics_client,
        provisioning_auth_client,
        user_info_client,
    };

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(), // DEV
            "http://localhost:8080".parse::<HeaderValue>().unwrap(), // DEBUG PROD
            "http://kubepods.com:8080".parse::<HeaderValue>().unwrap(), // PROD
        ])
        .allow_methods(Any)
        .allow_headers(Any);

    let http_buckets = [
        0.001, 0.005, 0.015, 0.05, 0.1, 0.2, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];
    PrometheusBuilder::new()
        .with_http_listener(([0, 0, 0, 0], 9000))
        .set_buckets_for_metric(
            Matcher::Suffix("duration_seconds".to_string()),
            &http_buckets,
        )?
        .install()?;

    let app = Router::new()
        .nest("/users", users::routes::auth())
        .nest("/desktops", desktops::routes::provisioning())
        .nest("/health", health::probes())
        .layer(from_fn(metrics::middleware))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
