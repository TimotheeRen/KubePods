use serde::Deserialize;

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
