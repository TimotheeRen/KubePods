mod domains;
mod errors;
mod handlers;
mod repositories;
mod services;

use std::env::var;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;

use crate::{
    handlers::{
        auth::{AuthImpl, user_auth::auth_service_server::AuthServiceServer},
        info::{InfoImpl, user_info::info_service_server::InfoServiceServer},
    },
    repositories::postgres::PostgresRepository,
    services::{auth::AuthServiceLayer, info::InfoServiceLayer},
};

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

    let info_service = InfoServiceLayer::new(repo);
    let info_service_handler = InfoImpl {
        service: info_service,
    };

    let addr = "0.0.0.0:50051".parse()?;
    Server::builder()
        .add_service(AuthServiceServer::new(auth_service_handler))
        .add_service(InfoServiceServer::new(info_service_handler))
        .serve(addr)
        .await?;

    Ok(())
}
