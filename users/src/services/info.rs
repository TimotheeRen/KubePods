use crate::{
    domains::info::UserAccount, errors::info::InfoError,
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

    pub async fn get_account(&self, username: String) -> Result<UserAccount, InfoError> {
        self.repo.get_account(username).await
    }

    pub async fn save_settings(
        &self,
        email: String,
        username: String,
        old_username: String,
    ) -> Result<(), InfoError> {
        self.repo
            .update_settings(email, username, old_username)
            .await
    }
}
