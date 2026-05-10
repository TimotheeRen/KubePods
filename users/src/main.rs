use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tonic::{Response, transport::Server};
use user::register_service_server::{RegisterService, RegisterServiceServer};

pub mod user {
    tonic::include_proto!("user");
}

pub struct RegisterImpl {
    pub pool: Pool<Postgres>,
}

#[tonic::async_trait]
impl RegisterService for RegisterImpl {
    async fn register(
        &self,
        request: tonic::Request<user::RegisterRequest>,
    ) -> std::result::Result<tonic::Response<user::RegisterResponse>, tonic::Status> {
        println!("{:?}", request);
        Ok(Response::new(user::RegisterResponse {
            success: true,
            message: "".to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pg_host = match std::env::var("DB_HOST") {
        Ok(val) => val,
        Err(_) => "postgres://postgres:password@localhost/users".to_string(), // Local dev
    };
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(&pg_host)
        .await?;
    let addr = "[::1]:50051".parse()?;
    let register_service = RegisterImpl { pool };
    Server::builder()
        .add_service(RegisterServiceServer::new(register_service))
        .serve(addr)
        .await?;

    Ok(())
}
