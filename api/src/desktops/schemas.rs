use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct createDesktop {
    pub name: String,
    pub distribution: String,
    pub desktop_environment: String,
}

#[derive(Serialize)]
pub struct DesktopItem {
    pub name: String,
    pub distribution: String,
    pub desktop_environment: String,
}
