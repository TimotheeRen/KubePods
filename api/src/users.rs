mod handlers;
pub mod routes;
mod schemas;

pub mod user_auth {
    tonic::include_proto!("auth");
}

pub mod user_info {
    tonic::include_proto!("info");
}
