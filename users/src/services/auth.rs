use crate::{
    domains::{
        auth::{LoginUser, User},
        error::AuthError,
    },
    handlers::auth::user::UsersTicks,
    repositories::auth::AuthRepository,
};

pub struct AuthServiceLayer<R: AuthRepository> {
    repo: R,
}

impl<R: AuthRepository> AuthServiceLayer<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<String, AuthError> {
        let register_user = User {
            username,
            email,
            password,
        };
        self.repo.create_user(&register_user).await?;
        let login_user = LoginUser {
            username: register_user.username,
            password: register_user.password,
        };
        let hash = self.repo.check_password(login_user.clone()).await?;
        self.repo
            .generate_token(login_user.username, login_user.password, &hash)
            .await
    }

    pub async fn login(&self, username: String, password: String) -> Result<String, AuthError> {
        let user = LoginUser { username, password };
        let hash = self.repo.check_password(user.clone()).await?;
        self.repo
            .generate_token(user.username, user.password, &hash)
            .await
    }

    pub async fn increment_chronometer(
        &self,
        users_ticks: Vec<UsersTicks>,
    ) -> Result<(), AuthError> {
        self.repo.increment_ticks(users_ticks).await
    }
}
