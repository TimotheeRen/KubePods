use crate::{
    errors::metrics::MetricsError,
    repositories::{
        kubernetes::KubernetesRepositoryInterface, postgres::PostgresRepositoryInterface,
    },
};

pub struct MetricsServiceLayer<K: KubernetesRepositoryInterface, P: PostgresRepositoryInterface> {
    kubernetes_repo: K,
    postgres_repo: P,
}

impl<K: KubernetesRepositoryInterface, P: PostgresRepositoryInterface> MetricsServiceLayer<K, P> {
    pub fn new(kubernetes_repo: K, postgres_repo: P) -> Self {
        Self {
            kubernetes_repo,
            postgres_repo,
        }
    }

    pub async fn get_remaining_desktops(&self, username: String) -> Result<u32, MetricsError> {
        self.postgres_repo.get_remaining_desktops(username).await
    }
}
