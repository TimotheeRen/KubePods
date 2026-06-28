use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Desktop {
    pub name: String,
    pub distribution: String,
    pub desktop_environment: String,
}
