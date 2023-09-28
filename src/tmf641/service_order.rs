use serde::{Deserialize, Serialize};
use uuid::Uuid;

// URL Path components
use super::LIB_PATH;
use super::MOD_PATH;

const SO_PATH: &str = "order";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ServiceOrder {
    id: String,
    href: String,
    description: Option<String>,
}

impl ServiceOrder {
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
