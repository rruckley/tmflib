//! Billing Account Module

use serde::{Deserialize, Serialize};

use crate::{HasId, HasLastUpdate, HasName, LIB_PATH, DateTime};
use tmflib_derive::{HasId, HasName, HasLastUpdate};
use crate::common::{
    money::Money,
    related_party::RelatedParty,
    contact::Contact,
};

use super::{AccountRef, AccountBalance, MOD_PATH};

const CLASS_PATH : &str = "account";

/// Billing Account
#[derive( Clone, Debug, Default, Deserialize, HasId, HasName, HasLastUpdate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementAccount {
    account_type: Option<String>,
    credit_limit: Option<Money>,
    /// Account Description
    pub description: Option<String>,
    /// Unique Identifier
    pub id: Option<String>,
    /// HTTP URI
    pub href: Option<String>,
    last_modified: Option<DateTime>,
    name: Option<String>,
    payment_status: Option<String>,
    rating_type: Option<String>,
    state: Option<String>,
    last_update : Option<DateTime>,
    related_party: Vec<RelatedParty>,
    contact: Option<Vec<Contact>>,
    account_balance: Option<Vec<AccountBalance>>,
}

impl SettlementAccount {
    /// Create new Billing Account
    pub fn new(name :impl Into<String>) -> SettlementAccount {
        SettlementAccount {
            name: Some(name.into()),
            ..SettlementAccount::create()
        }
    }
}

impl From<SettlementAccount> for AccountRef {
    fn from(value: SettlementAccount) -> Self {
        AccountRef {
            id : value.get_id(),
            href: value.get_href(),
            name: value.get_name(),
            description : value.description.clone(),
        }
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

#[cfg(test)]
mod test {
    use super::*;

    const ACCOUNT : &str = "SettlementAccount";

    #[test]
    fn test_settlement_account_new_name() {
        let account = SettlementAccount::new(ACCOUNT);

        assert_eq!(account.name,Some(ACCOUNT.into()));
    }
}