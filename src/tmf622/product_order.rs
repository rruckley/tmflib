//! Product Order Module

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// URL Path components
use super::LIB_PATH;
use super::MOD_PATH;

const PO_PATH: &str = "order";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ProductOrder {
    id: String,
    href: String,
}

impl ProductOrder {
    pub fn new() -> ProductOrder {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}", LIB_PATH, MOD_PATH, PO_PATH, id);
        ProductOrder { id, href }
    }
}
