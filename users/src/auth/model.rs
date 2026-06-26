use serde::{Deserialize, Serialize};

pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: String,
    pub role: String,
    pub exp: i64,
}

pub struct UserTicks {
    pub username: String,
    pub ticks: i32,
}
