use tonic::{Response, Status};
use user_auth::auth_service_server::AuthService;

use crate::{
    errors::auth::AuthError, repositories::postgres::PostgresRepository,
    services::auth::AuthServiceLayer,
};

pub mod user_auth {
    tonic::include_proto!("auth");
}

pub struct AuthImpl {
    pub service: AuthServiceLayer<PostgresRepository>,
}

#[tonic::async_trait]
impl AuthService for AuthImpl {
    async fn register(
        &self,
        request: tonic::Request<user_auth::RegisterRequest>,
    ) -> std::result::Result<tonic::Response<user_auth::RegisterResponse>, tonic::Status> {
        let req = request.into_inner();

        let token = self
            .service
            .register(req.username, req.email, req.password)
            .await
            .map_err(|e| match e {
                AuthError::UserAlreadyExists => Status::already_exists("Username already taken"),
                _ => Status::internal("Internal server error"),
            })?;

        Ok(Response::new(user_auth::RegisterResponse { token }))
    }

    async fn login(
        &self,
        request: tonic::Request<user_auth::LoginRequest>,
    ) -> std::result::Result<tonic::Response<user_auth::LoginResponse>, tonic::Status> {
        let req = request.into_inner();
        let token = self
            .service
            .login(req.username, req.password)
            .await
            .map_err(|e| match e {
                AuthError::WrongPassword => Status::unauthenticated("Wrong credentials"),
                _ => Status::internal("Internal server error"),
            })?;
        Ok(Response::new(user_auth::LoginResponse { token }))
    }
}
