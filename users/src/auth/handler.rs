use tonic::{Response, Status};
use user::auth_service_server::AuthService;

use crate::auth::{
    error::{CheckPasswordError, CreateUserError},
    repo::PostgresAuthRepository,
    service::AuthServiceLayer,
};

pub mod user {
    tonic::include_proto!("user");
}

pub struct AuthImpl {
    pub service: AuthServiceLayer<PostgresAuthRepository>,
}

#[tonic::async_trait]
impl AuthService for AuthImpl {
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

    async fn login(
        &self,
        request: tonic::Request<user::LoginRequest>,
    ) -> std::result::Result<tonic::Response<user::LoginResponse>, tonic::Status> {
        let req = request.into_inner();
        self.service
            .login(req.username, req.password)
            .await
            .map_err(|e| match e {
                CheckPasswordError::WrongPassword => Status::unauthenticated("Wrong credentials"),
                CheckPasswordError::DatabaseError => Status::internal("Internal server error"),
            })?;
        Ok(Response::new(user::LoginResponse {
            token: "".to_string(),
        }))
    }
}
