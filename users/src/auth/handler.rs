use tonic::{Response, Status};
use user::auth_service_server::AuthService;

use crate::auth::{error::AuthError, repo::PostgresAuthRepository, service::AuthServiceLayer};

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
        let req = request.into_inner();

        let token = self
            .service
            .register(req.username, req.email, req.password)
            .await
            .map_err(|e| match e {
                AuthError::UserAlreadyExists => Status::already_exists("Username already taken"),
                _ => Status::internal("Internal server error"),
            })?;

        Ok(Response::new(user::RegisterResponse { token }))
    }

    async fn login(
        &self,
        request: tonic::Request<user::LoginRequest>,
    ) -> std::result::Result<tonic::Response<user::LoginResponse>, tonic::Status> {
        let req = request.into_inner();
        let token = self
            .service
            .login(req.username, req.password)
            .await
            .map_err(|e| match e {
                AuthError::WrongPassword => Status::unauthenticated("Wrong credentials"),
                _ => Status::internal("Internal server error"),
            })?;
        Ok(Response::new(user::LoginResponse { token }))
    }

    async fn increment_chronometer(
        &self,
        request: tonic::Request<user::IncrementChronometerRequest>,
    ) -> std::result::Result<tonic::Response<user::IncrementChronometerResponse>, tonic::Status>
    {
        let req = request.into_inner();
        println!("{:?}", req);
        self.service.increment_chronometer(req.users_ticks).await;

        Ok(Response::new(user::IncrementChronometerResponse {}))
    }
}
