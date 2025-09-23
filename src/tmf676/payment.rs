//! TMF676 Payment Module
//!

use crate::{HasId, HasName, Uri};
use serde::{Deserialize, Serialize};

use tmflib_derive::{HasId, HasName};

use super::MOD_PATH;

const CLASS_PATH: &str = "payment";

/// A Payment
#[derive(Clone, Debug, Default, HasId, HasName, Serialize, Deserialize)]
pub struct Payment {
    /// Unique Id
    pub id: Option<String>,
    /// HTTP Uri
    pub href: Option<Uri>,
    /// Refund Name
    pub name: Option<String>,
}
