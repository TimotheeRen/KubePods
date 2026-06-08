use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Desktop {
    pub name: String,
    pub distribution: String,
    pub desktop_environement: String,
}
