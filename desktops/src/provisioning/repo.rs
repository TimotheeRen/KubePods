use sqlx::{Pool, Postgres};

use crate::provisioning::{error::ProvisioningError, model::Desktop};

pub trait ProvisioningRepository {
    async fn create_desktop(&self, desktop: &Desktop) -> Result<(), ProvisioningError>;
}

pub struct PostgresProvioningRepository {
    pub pool: Pool<Postgres>,
}

impl PostgresProvioningRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

impl ProvisioningRepository for PostgresProvioningRepository {
    async fn create_desktop(&self, desktop: &Desktop) -> Result<(), ProvisioningError> {
        Ok(())
    }
}
