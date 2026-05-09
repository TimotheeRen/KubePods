mod handlers;
pub mod routes;
mod schemas;

pub mod user {
    tonic::include_proto!("user");
}
