use crate::{
    domains::auth::{LoginUser, User},
    errors::auth::AuthError,
    repositories::postgres::PostgresRepositoryInterface,
};

pub struct AuthServiceLayer<R: PostgresRepositoryInterface> {
    repo: R,
}

impl<R: PostgresRepositoryInterface> AuthServiceLayer<R> {
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
}
