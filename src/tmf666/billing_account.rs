//! Billing Account Module

use serde::{Deserialize, Serialize};

use crate::{HasId,HasLastUpdate,CreateTMF,CreateTMFWithTime, LIB_PATH};
use tmflib_derive::HasId;

use super::MOD_PATH;

const CLASS_PATH : &str = "account";

/// Billing Account
#[derive( Clone, Debug, Default, Deserialize, HasId, Serialize)]
pub struct BillingAccount {
    id: Option<String>,
    href: Option<String>,
    name: String,
    last_update : Option<String>,
}

impl BillingAccount {
    /// Create new Billing Account
    pub fn new(name :impl Into<String>) -> BillingAccount {
        let mut account = BillingAccount::create();
        account.name = name.into();
        account
    }
}

impl HasLastUpdate for BillingAccount {
    fn set_last_update(&mut self, time : impl Into<String>) {
        self.last_update = Some(time.into());
    }
}
impl CreateTMFWithTime<BillingAccount> for BillingAccount {}