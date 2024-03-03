//! Party Module
//!
//!

use serde::{Deserialize, Serialize};
/// Related Party
#[deprecated]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedPartyXXX {
    id: String,
    href: String,
    name: String,
    role: String,
}
