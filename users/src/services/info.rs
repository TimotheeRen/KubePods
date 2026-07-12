use crate::{
    domains::info::UserAccount,
    errors::info::InfoError,
    repositories::{external::ExternalRepositoryInterface, postgres::PostgresRepositoryInterface},
};

pub struct InfoServiceLayer<P: PostgresRepositoryInterface, E: ExternalRepositoryInterface> {
    postgres_repo: P,
    external_repo: E,
}

impl<P: PostgresRepositoryInterface, E: ExternalRepositoryInterface> InfoServiceLayer<P, E> {
    pub fn new(postgres_repo: P, external_repo: E) -> Self {
        Self {
            postgres_repo,
            external_repo,
        }
    }

    pub async fn get_remaining_time(&self, username: String) -> Result<u32, InfoError> {
        self.postgres_repo.get_remaining_time(username).await
    }

    pub async fn get_account(&self, username: String) -> Result<UserAccount, InfoError> {
        self.postgres_repo.get_account(username).await
    }

    pub async fn save_settings(
        &self,
        email: String,
        username: String,
        old_username: String,
    ) -> Result<(), InfoError> {
        self.postgres_repo
            .update_settings(email, username.clone(), old_username.clone())
            .await;
        self.external_repo
            .update_settings(username, old_username)
            .await;
        Ok(())
    }
}
