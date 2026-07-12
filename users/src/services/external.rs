use crate::{
    errors::auth::AuthError, handlers::external::user_external::UsersTicks,
    repositories::postgres::PostgresRepositoryInterface,
};

pub struct ExternalServiceLayer<R: PostgresRepositoryInterface> {
    repo: R,
}

impl<R: PostgresRepositoryInterface> ExternalServiceLayer<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn increment_chronometer(
        &self,
        users_ticks: Vec<UsersTicks>,
    ) -> Result<(), AuthError> {
        self.repo.increment_ticks(users_ticks).await
    }
}
