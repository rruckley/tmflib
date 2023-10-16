use serde::{Deserialize, Serialize};
use uuid::Uuid;

// URL Path components
use super::LIB_PATH;
use super::MOD_PATH;

const SO_PATH: &str = "order";

/// Service Order Object
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceOrder {
    id: String,
    href: String,
    description: Option<String>,
}

impl ServiceOrder {
    /// Create a new service order object
    pub fn new() -> ServiceOrder {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}", LIB_PATH, MOD_PATH, SO_PATH, id);
        ServiceOrder {
            id,
            href,
            description: None,
        }
    }
}
