use serde::Deserialize;

#[derive(Deserialize)]
pub struct createDesktop {
    pub name: String,
    pub distribution: String,
    pub desktop_environment: String,
}
