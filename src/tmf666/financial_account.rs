//! Financial Account Module

use serde::{Deserialize, Serialize};

use crate::{
    HasId, 
    HasLastUpdate, 
    HasName, 
    LIB_PATH, 
    DateTime,
};
use tmflib_derive::{HasId, HasName, HasLastUpdate};

use crate::common::{
    money::Money,
    related_party::RelatedParty,
    contact::Contact,
};
use super::{
    AccountBalance, 
    AccountRelationship,
    AccountTaxExemption,
    MOD_PATH
};

const CLASS_PATH : &str = "account";

/// Financial Account Reference
#[derive( Clone, Debug, Default, Deserialize, Serialize)]
pub struct FinancialAccountRef {
    id : String,
    href: String,
    name: String,
    account_balance: Option<AccountBalance>,
}

impl From<FinancialAccount> for FinancialAccountRef {
    fn from(value: FinancialAccount) -> Self {
        FinancialAccountRef {
            id: value.get_id(),
            href: value.get_href(),
            name: value.get_name(),
            account_balance: value.account_balance.as_ref().unwrap().first().cloned(),
        }
    }
}

/// Finiancial Account
#[derive( Clone, Debug, Default, Deserialize, HasId, HasName, HasLastUpdate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialAccount {
    account_type: String,
    credit_limit: Option<Money>,
    description: Option<String>,
    /// HTTP URI
    pub href : Option<String>,
    /// Unique Id
    pub id : Option<String>,
    last_update: Option<DateTime>,
    /// Name of this account
    pub name : Option<String>,
    /// Status of this account
    pub state : Option<String>,
    // Referenced data
    related_party: Option<Vec<RelatedParty>>,
    contact: Option<Vec<Contact>>,
    account_balance: Option<Vec<AccountBalance>>,
    account_relationship: Option<Vec<AccountRelationship>>,
    tax_exemption: Option<Vec<AccountTaxExemption>>,
}