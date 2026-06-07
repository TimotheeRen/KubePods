mod provisioning;
use crate::provisioning::{
    handler::{ProvisioningImpl, user::provisioning_service_server::ProvisioningServiceServer},
    repo::PostgresProvioningRepository,
    service::ProvisioningServiceLayer,
};
use dotenvy::{dotenv, var};
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;

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

    let addr = "0.0.0.0:50051".parse()?;
    let repo = PostgresProvioningRepository::new(pool);
    let service = ProvisioningServiceLayer::new(repo);
    let provisioning_service = ProvisioningImpl { service };

    Server::builder()
        .add_service(ProvisioningServiceServer::new(provisioning_service))
        .serve(addr)
        .await?;

    Ok(())
}
