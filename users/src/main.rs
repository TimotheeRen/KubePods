mod domains;
mod handlers;
mod repositories;
mod services;

use std::env::var;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;

use crate::{
    handlers::auth::{AuthImpl, user_auth::auth_service_server::AuthServiceServer},
    repositories::auth::PostgresAuthRepository,
    services::auth::AuthServiceLayer,
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

    let repo = PostgresAuthRepository::new(pool);
    let service = AuthServiceLayer::new(repo);
    let register_service = AuthImpl { service };

    let addr = "0.0.0.0:50051".parse()?;
    Server::builder()
        .add_service(AuthServiceServer::new(register_service))
        .serve(addr)
        .await?;

    Ok(())
}
