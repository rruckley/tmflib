//! Billing Account Module

use serde::{Deserialize, Serialize};

use crate::{HasId, HasLastUpdate, HasName, LIB_PATH, DateTime};
use tmflib_derive::{HasId, HasName, HasLastUpdate};
use crate::common::{
    money::Money,
    related_party::RelatedParty,
    contact::Contact,
};

use super::{
    AccountRef, 
    AccountBalance, 
    AccountTaxExemption,
    financial_account::FinancialAccountRef,
    MOD_PATH,
    PaymentPlan,
    PaymentMethodRef,
};

const CLASS_PATH : &str = "account";

/// Billing Account
#[derive( Clone, Debug, Default, Deserialize, HasId, HasName, HasLastUpdate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_limit: Option<Money>,
    /// Account Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Unique Identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTTP URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rating_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update : Option<DateTime>,
    related_party: Vec<RelatedParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<Vec<Contact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_balance: Option<Vec<AccountBalance>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_exemption: Option<Vec<AccountTaxExemption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    financial_account: Option<FinancialAccountRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_plan: Option<Vec<PaymentPlan>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_payment_method: Option<PaymentMethodRef>,
}

impl BillingAccount {
    /// Create new Billing Account
    pub fn new(name :impl Into<String>) -> BillingAccount {
        let mut account = BillingAccount::create();
        account.name = Some(name.into());
        account
    }
}

impl From<BillingAccount> for AccountRef {
    fn from(value: BillingAccount) -> Self {
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

impl From<BillingAccount> for BillingAccountRef {
    fn from(value: BillingAccount) -> Self {
        BillingAccountRef {
            id : value.id.unwrap_or_default(),
            href : value.href.unwrap_or_default(),
            name: value.name.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const ACCOUNT : &str = "BillingAccount";

    #[test]
    fn test_billing_account_new_name() {
        let account = BillingAccount::new(ACCOUNT);

        assert_eq!(account.name,Some(ACCOUNT.into()));
    }
}