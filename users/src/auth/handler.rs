use sqlx::{Pool, Postgres, query};
use tonic::Response;
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

        let message = match self
            .service
            .register(req.username, req.email, req.password)
            .await
        {
            Ok(_) => "".to_string(),
            Err(CreateUserError::UserAlreadyExists) => "This username is already taken".to_string(),
            Err(CreateUserError::DatabaseError) => "An error occured".to_string(),
        };

        Ok(Response::new(user::RegisterResponse {
            success: true,
            message,
        }))
    }
}
