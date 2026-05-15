use crate::auth::{error::CreateUserError, model::User, repo::AuthRepository};

pub struct AuthService<R: AuthRepository> {
    repo: R,
}

impl<R: AuthRepository> AuthService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<(), CreateUserError> {
        let user = User {
            username,
            email,
            password,
        };
        self.repo.create_user(&user).await?;

        Ok(())
    }
}
