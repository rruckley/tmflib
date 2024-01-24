//! Billing Account Module

use serde::{Deserialize, Serialize};

use crate::{HasId, HasLastUpdate, HasName, CreateTMF, CreateTMFWithTime, LIB_PATH};
use tmflib_derive::{HasId, HasName, HasLastUpdate};

use super::MOD_PATH;

const CLASS_PATH : &str = "account";

/// Billing Account
#[derive( Clone, Debug, Default, Deserialize, HasId, HasName, HasLastUpdate, Serialize)]
pub struct BillingAccount {
    id: Option<String>,
    href: Option<String>,
    name: Option<String>,
    last_update : Option<String>,
}

impl BillingAccount {
    /// Create new Billing Account
    pub fn new(name :impl Into<String>) -> BillingAccount {
        let mut account = BillingAccount::create();
        account.name = Some(name.into());
        account
    }
}

/// Billing Account Reference
#[derive( Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingAccountRef {
    /// Referenced Id
    id: String,
    /// Referenced HREF
    href: String,
    /// Referenced Name
    name: String,
}

impl From<BillingAccount> for BillingAccountRef {
    fn from(value: BillingAccount) -> Self {
        BillingAccountRef {
            id : value.id.unwrap_or_default(),
            href : value.href.unwrap_or_default(),
            name: value.name.unwrap_or_default(),
        }
    }
}