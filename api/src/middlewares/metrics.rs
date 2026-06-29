use std::time::Instant;

use axum::{
    extract::Request,
    middleware::Next,
    response::{self, Response},
};
use metrics::{counter, describe_counter, describe_histogram, histogram};

pub async fn middleware(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let path = request.uri().path().to_string();
    let response = next.run(request).await;
    let duration = start.elapsed().as_secs_f64();
    let status = response.status().as_u16();
    counter!("http_requests_total", "path" => path.clone(), "status" => status.to_string())
        .increment(1);
    describe_counter!(
        "http_request_duration_total",
        "Total number of incoming http requests"
    );
    histogram!("http_request_duration_seconds", "path" => path).record(duration);
    describe_histogram!(
        "http_request_duration_seconds",
        "Incoming http requests durations"
    );
    response
}
