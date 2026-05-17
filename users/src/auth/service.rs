use crate::auth::{
    error::{CheckPasswordError, CreateUserError},
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
    ) -> Result<(), CreateUserError> {
        let user = User {
            username,
            email,
            password,
        };
        self.repo.create_user(&user).await?;

        Ok(())
    }

    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<(), CheckPasswordError> {
        let user = LoginUser { username, password };
        self.repo.check_password(user).await?;
        Ok(())
    }
}
