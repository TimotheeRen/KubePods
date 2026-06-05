use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(group = "kubepods.com", version = "v1", kind = "Desktop", namespaced)]
pub struct DesktopSpec {
    pub name: String,
    pub id: String,
    pub image: String,
    pub max_ram: String,
    pub max_cpu: String,
}
