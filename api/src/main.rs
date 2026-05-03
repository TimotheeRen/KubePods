use axum::{Router, routing::{get, post}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let app = Router::new()
        .route("/register", post(register));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn register() -> String {
    println!("Received a registering demand");
    return "Registered.".to_string()
}
