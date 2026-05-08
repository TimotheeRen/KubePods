use tonic::{transport::Server, Response};
use user::register_service_server::{RegisterService, RegisterServiceServer};

pub mod user {
    tonic::include_proto!("user");
}

#[derive(Default)]
pub struct RegisterImpl {}

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
    let addr = "[::1]:50051".parse()?;
    Server::builder()
        .add_service(RegisterServiceServer::new(RegisterImpl::default()))
        .serve(addr)
        .await?;

    Ok(())
}
