use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RemainingTime {
    pub utilization: u8,
    pub remaining: u8,
}
