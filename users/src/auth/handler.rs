use sqlx::{Pool, Postgres, query};
use tonic::{Response, Status};
use user::register_service_server::RegisterService;

use crate::auth::{error::CreateUserError, repo::PostgresAuthRepository, service::AuthService};

pub mod user {
    tonic::include_proto!("user");
}

pub struct RegisterImpl {
    pub service: AuthService<PostgresAuthRepository>,
}

#[tonic::async_trait]
impl RegisterService for RegisterImpl {
    async fn register(
        &self,
        request: tonic::Request<user::RegisterRequest>,
    ) -> std::result::Result<tonic::Response<user::RegisterResponse>, tonic::Status> {
        println!("{:?}", request);
        let req = request.into_inner();

        self.service
            .register(req.username, req.email, req.password)
            .await
            .map_err(|e| match e {
                CreateUserError::UserAlreadyExists => {
                    Status::already_exists("Username already taken")
                }
                CreateUserError::DatabaseError => Status::internal("Internal server error"),
                CreateUserError::HashPasswordError => {
                    Status::internal("An error occured when hasing the password")
                }
            })?;

        Ok(Response::new(user::RegisterResponse {
            success: true,
            message: "".to_string(),
        }))
    }
}
