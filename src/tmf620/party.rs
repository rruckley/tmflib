//! Party Module
//!
//!

use serde::{Deserialize, Serialize};
/// Related Party
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedParty {
    id: String,
    href: String,
    name: String,
    role: String,
}
