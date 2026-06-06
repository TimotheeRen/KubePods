use crate::auth::{
    error::AuthError,
    model::{LoginUser, User},
    repo::AuthRepository,
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
    ) -> Result<(), AuthError> {
        let user = User {
            username,
            email,
            password,
        };
        self.repo.create_user(&user).await?;

        Ok(())
    }

    pub async fn login(&self, username: String, password: String) -> Result<String, AuthError> {
        let user = LoginUser { username, password };
        let hash = self.repo.check_password(user.clone()).await?;
        self.repo
            .generate_token(user.username, user.password, &hash)
            .await
    }
}
