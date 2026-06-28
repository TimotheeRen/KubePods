use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateDesktop {
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

#[derive(Deserialize)]
pub struct DeleteDesktop {
    pub name: String,
}

#[derive(Serialize)]
pub struct GetRemainingDesktops {
    pub created: u8,
    pub remaining: u8,
}
