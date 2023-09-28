//! Party Module
//!
//!

use serde::{Deserialize, Serialize};
/// Related Party
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedParty {
    id: String,
    href: String,
    name: String,
    role: String,
}
