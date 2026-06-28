use crate::{
    errors::{auth::AuthError, info::InfoError},
    handlers::auth::user_auth::UsersTicks,
    repositories::postgres::PostgresRepositoryInterface,
};

pub struct InfoServiceLayer<R: PostgresRepositoryInterface> {
    repo: R,
}

impl<R: PostgresRepositoryInterface> InfoServiceLayer<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_remaining_time(&self, username: String) -> Result<u32, InfoError> {
        self.repo.get_remaining_time(username).await
    }
}
