use std::env::var;

use dotenvy::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions, query};
use tonic::{Response, Status, transport::Server};
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
        let req = request.into_inner();
        query("INSERT INTO users (username, email, password) VALUES ($1, $2, $3)")
            .bind(&req.username)
            .bind(&req.email)
            .bind(&req.password)
            .execute(&self.pool)
            .await
            .map_err(|e| tonic::Status::internal(format!("Database error: {e}")))?;
        Ok(Response::new(user::RegisterResponse {
            success: true,
            message: "".to_string(),
        }))
    }
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

    let addr = "0.0.0.0:50051".parse()?;
    let register_service = RegisterImpl { pool };

    Server::builder()
        .add_service(RegisterServiceServer::new(register_service))
        .serve(addr)
        .await?;

    Ok(())
}
