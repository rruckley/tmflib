//! TMF676 Refund Module

use crate::{HasId, HasName, Uri};
use serde::{Deserialize, Serialize};

use tmflib_derive::{HasId, HasName};

use super::MOD_PATH;

const CLASS_PATH: &str = "refund";

/// A Refund
#[derive(Clone, Debug, Default, HasId, HasName, Serialize, Deserialize)]
pub struct Refund {
    /// Unique Id
    pub id: Option<String>,
    /// HTTP Uri
    pub href: Option<Uri>,
    /// Refund Name
    pub name: Option<String>,
}
