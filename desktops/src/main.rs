mod daemon;
mod domains;
mod errors;
mod handlers;
mod repositories;
mod services;

use dotenvy::{dotenv, var};
use kube::Client;
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;

use crate::{
    handlers::{
        metrics::{MetricsHandler, desktops_metrics::metrics_service_server::MetricsServiceServer},
        provisioning::{
            ProvisioningHandler,
            desktops_provisioning::provisioning_service_server::ProvisioningServiceServer,
        },
    },
    repositories::{kubernetes::KubernetesRepository, postgres::PostgresRepository},
    services::{metrics::MetricsServiceLayer, provisioning::ProvisioningServiceLayer},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pg_host = match var("DB_HOST") {
        Ok(val) => val,
        Err(_) => match var("DB_PASS") {
            Ok(val) => {
                format!("postgresql://app:{val}@localhost:5433/app")
            }
            Err(_) => return Err("You need to add a .env with the db password in $DB_PASS".into()),
        },
    };

    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(&pg_host)
        .await?;

    tokio::spawn(daemon::increment(pool.clone()));

    let client = Client::try_default().await?;

    let kubernetes_repo = KubernetesRepository::new(client);
    let postgres_repo = PostgresRepository::new(pool);

    let provisioning_service =
        ProvisioningServiceLayer::new(kubernetes_repo.clone(), postgres_repo.clone());
    let provisioning_service_handler = ProvisioningHandler {
        service: provisioning_service,
    };

    let metrics_service = MetricsServiceLayer::new(kubernetes_repo, postgres_repo);
    let metrics_service_handler = MetricsHandler {
        service: metrics_service,
    };

    let addr = "0.0.0.0:50052".parse()?;
    Server::builder()
        .add_service(ProvisioningServiceServer::new(provisioning_service_handler))
        .add_service(MetricsServiceServer::new(metrics_service_handler))
        .serve(addr)
        .await?;

    Ok(())
}
