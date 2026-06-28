mod handlers;
pub mod routes;
mod schemas;

pub mod desktops_provisioning {
    tonic::include_proto!("provisioning");
}

pub mod desktops_metrics {
    tonic::include_proto!("metrics");
}
