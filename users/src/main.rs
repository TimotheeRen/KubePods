mod domains;
mod errors;
mod handlers;
mod repositories;
mod services;

use std::{env::var, time::Duration};

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tonic::transport::{Channel, Server};

use crate::{
    desktops_metrics::metrics_service_client::MetricsServiceClient,
    handlers::{
        auth::{AuthImpl, user_auth::auth_service_server::AuthServiceServer},
        external::{ExternalImpl, user_external::external_service_server::ExternalServiceServer},
        info::{InfoImpl, user_info::info_service_server::InfoServiceServer},
    },
    repositories::{
        external::{ExternalRepository, ExternalRepositoryInterface},
        postgres::PostgresRepository,
    },
    services::{auth::AuthServiceLayer, external::ExternalServiceLayer, info::InfoServiceLayer},
};

pub mod desktops_metrics {
    tonic::include_proto!("metrics");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pg_host = match var("DB_HOST") {
        Ok(val) => val,
        Err(_) => match var("DB_PASS") {
            Ok(val) => {
                format!("postgresql://app:{val}@localhost:5432/app")
            }
            Err(_) => return Err("You need to add a .env with the db password in $DB_PASS".into()),
        },
    };

    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(&pg_host)
        .await?;

    let repo = PostgresRepository::new(pool);
    let auth_service = AuthServiceLayer::new(repo.clone());
    let auth_service_handler = AuthImpl {
        service: auth_service,
    };

    let external_repo = ExternalRepository::new().await?;
    let info_service = InfoServiceLayer::new(repo.clone(), external_repo);
    let info_service_handler = InfoImpl {
        service: info_service,
    };

    let external_service = ExternalServiceLayer::new(repo);
    let external_service_handler = ExternalImpl {
        service: external_service,
    };

    let addr = "0.0.0.0:50051".parse()?;
    Server::builder()
        .add_service(AuthServiceServer::new(auth_service_handler))
        .add_service(InfoServiceServer::new(info_service_handler))
        .add_service(ExternalServiceServer::new(external_service_handler))
        .serve(addr)
        .await?;

    Ok(())
}
