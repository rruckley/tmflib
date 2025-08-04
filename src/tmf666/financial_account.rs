//! Financial Account Module

use serde::{Deserialize, Serialize};

use crate::{DateTime, HasId, HasLastUpdate, HasName, LIB_PATH};
use tmflib_derive::{HasId, HasLastUpdate, HasName};

use super::{AccountBalance, AccountRelationship, AccountTaxExemption, MOD_PATH};
use crate::common::{contact::Contact, money::Money, related_party::RelatedParty};

const CLASS_PATH: &str = "account";

/// Financial Account Reference
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FinancialAccountRef {
    id: String,
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
            account_balance: Some(value.get_balance()),
        }
    }
}

/// Finiancial Account
#[derive(Clone, Debug, Default, Deserialize, HasId, HasName, HasLastUpdate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialAccount {
    account_type: String,
    credit_limit: Option<Money>,
    description: Option<String>,
    /// HTTP URI
    pub href: Option<String>,
    /// Unique Id
    pub id: Option<String>,
    last_update: Option<DateTime>,
    /// Name of this account
    pub name: Option<String>,
    /// Status of this account
    pub state: Option<String>,
    // Referenced data
    related_party: Option<Vec<RelatedParty>>,
    contact: Option<Vec<Contact>>,
    account_balance: Option<Vec<AccountBalance>>,
    account_relationship: Option<Vec<AccountRelationship>>,
    tax_exemption: Option<Vec<AccountTaxExemption>>,
}

impl FinancialAccount {
    /// Get summed balance accross all AccountBalance records
    pub fn get_balance(&self) -> AccountBalance {
        let total = match self.account_balance.as_ref() {
            Some(v) => {
                let mut out = Money::from(0.0);
                v.iter().for_each(|ab| {
                    out += match ab.amount.as_ref() {
                        Some(m) => m.clone(),
                        None => Money::default(),
                    }
                });
                out
            }
            None => Money::from(0.0),
        };
        AccountBalance {
            amount: Some(total),
            balance_type: String::from("total"),
            valid_for: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{HasId, HasName};
    const ACC_NAME: &str = "Financial Account";

    #[test]
    fn test_ref_from_financial() {
        let mut financial = FinancialAccount::default();
        financial.generate_id();
        financial.set_name(ACC_NAME);

        let fin_ref = FinancialAccountRef::from(financial.clone());

        assert_eq!(fin_ref.id, financial.get_id());
        assert_eq!(fin_ref.href, financial.get_href());
        assert_eq!(fin_ref.name, financial.get_name());
    }
}
