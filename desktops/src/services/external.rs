use crate::{
    errors::external::ExternalError,
    repositories::{
        kubernetes::KubernetesRepositoryInterface, postgres::PostgresRepositoryInterface,
    },
};

pub struct ExternalServiceLayer<K: KubernetesRepositoryInterface, P: PostgresRepositoryInterface> {
    kubernetes_repo: K,
    postgres_repo: P,
}

impl<K: KubernetesRepositoryInterface, P: PostgresRepositoryInterface> ExternalServiceLayer<K, P> {
    pub fn new(kubernetes_repo: K, postgres_repo: P) -> Self {
        Self {
            kubernetes_repo,
            postgres_repo,
        }
    }

    pub async fn change_desktops_user(
        &self,
        username: String,
        old_username: String,
    ) -> Result<(), ExternalError> {
        self.postgres_repo
            .change_desktops_user(username, old_username)
            .await
    }
}
