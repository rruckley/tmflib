//! External Identifier Module

use serde::{Deserialize,Serialize};
use crate::{HasId,Uri,LIB_PATH};
use tmflib_derive::HasId;

const CLASS_PATH : &str = "external";
use super::MOD_PATH;

/// External Identifier Reference
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIdentifier {
    external_identifier_type: String,
    href: Option<Uri>,
    id: Option<String>,
    owner: String,
}