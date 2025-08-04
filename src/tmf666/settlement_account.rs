//! Billing Account Module

use serde::{Deserialize, Serialize};

use crate::common::{contact::Contact, money::Money, related_party::RelatedParty};
use crate::{DateTime, HasDescription, HasId, HasLastUpdate, HasName, LIB_PATH};
use tmflib_derive::{HasDescription, HasId, HasLastUpdate, HasName};

use super::{
    financial_account::FinancialAccountRef, AccountBalance, AccountRef, AccountTaxExemption,
    PaymentMethodRef, PaymentPlan, MOD_PATH,
};

const CLASS_PATH: &str = "account";

/// Billing Account
#[derive(
    Clone, Debug, Default, Deserialize, HasId, HasName, HasLastUpdate, HasDescription, Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SettlementAccount {
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
    last_update: Option<DateTime>,
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

impl SettlementAccount {
    /// Create new Billing Account
    pub fn new(name: impl Into<String>) -> SettlementAccount {
        SettlementAccount {
            name: Some(name.into()),
            ..SettlementAccount::create()
        }
    }
}

impl From<SettlementAccount> for AccountRef {
    fn from(value: SettlementAccount) -> Self {
        AccountRef {
            id: value.get_id(),
            href: value.get_href(),
            name: value.get_name(),
            description: value.description.clone(),
        }
    }
}

/// Billing Account Reference
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
    use crate::HasId;

    const ACCOUNT_NAME: &str = "SettlementAccount";

    #[test]
    fn test_settlement_account_new_name() {
        let account = SettlementAccount::new(ACCOUNT_NAME);

        assert_eq!(account.name, Some(ACCOUNT_NAME.into()));
    }

    #[test]
    fn test_accountref_from_settlementaccount() {
        let settlementaccount = SettlementAccount::new(ACCOUNT_NAME);

        let accountref = AccountRef::from(settlementaccount.clone());

        assert_eq!(accountref.id, settlementaccount.get_id());
        assert_eq!(accountref.href, settlementaccount.get_href());
        assert_eq!(accountref.name, settlementaccount.get_name());
    }
}
