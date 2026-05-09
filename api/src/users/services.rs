use crate::users::schemas;

pub async fn register_user(user: schemas::RegisterUser) {
    println!("Received a register request: {}", user.username);
}
