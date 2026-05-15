mod auth;
use std::env::var;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;

use crate::auth::{
    handler::{RegisterImpl, user::register_service_server::RegisterServiceServer},
    repo::PostgresAuthRepository,
    service::AuthService,
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
    let service = AuthService::new(repo);

    let addr = "0.0.0.0:50051".parse()?;
    let register_service = RegisterImpl { service };

    Server::builder()
        .add_service(RegisterServiceServer::new(register_service))
        .serve(addr)
        .await?;

    Ok(())
}
