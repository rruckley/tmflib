//! Party Account Module

use serde::{Deserialize, Serialize};

use crate::{
    HasId, 
    HasDescription,
    HasLastUpdate, 
    HasName,
    HasRelatedParty,
    LIB_PATH, 
    DateTime,
};
use tmflib_derive::{
    HasId, 
    HasDescription,
    HasName, 
    HasLastUpdate,
    HasRelatedParty,
};

use crate::common::{
    money::Money,
    related_party::RelatedParty,
    contact::Contact,
};
use super::{
    AccountBalance,
    AccountRef,
    AccountTaxExemption,
    financial_account::FinancialAccountRef,
    MOD_PATH,
    PaymentPlan,
    PaymentMethodRef,
};

const CLASS_PATH : &str = "account";

/// Party Account
#[derive( Clone, Debug, Default, Deserialize, HasId, HasName, HasLastUpdate, HasRelatedParty, HasDescription, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartyAccount {
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
    last_update: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    // Referenced 
    related_party: Option<Vec<RelatedParty>>,
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

impl From<PartyAccount> for AccountRef {
    fn from(value: PartyAccount) -> Self {
        AccountRef {
            id : value.get_id(),
            href: value.get_href(),
            name: value.get_name(),
            description : value.description.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{HasId, HasName};

    use super::PartyAccount;
    use crate::tmf666::AccountRef;

    const PARTYACC_ID : &str = "PA123";
    const PARTYACC_NAME : &str =  "PartyAccountName";
    const PARTYACC_JSON : &str = "{
        \"name\" : \"PartyAccountName\",
        \"description\" : \"Description\"
    }";

    #[test]
    fn test_partyacc_deserialize() {
        let partyacc : PartyAccount = serde_json::from_str(PARTYACC_JSON).unwrap();

        assert_eq!(partyacc.description.is_some(),true);
        assert_eq!(partyacc.description.unwrap().as_str(),"Description");
    }

    #[test]
    fn test_partyacc_hasname() {
        let mut partyacc = PartyAccount::default();

        partyacc.set_id(PARTYACC_ID);
        partyacc.set_name(PARTYACC_NAME);

        assert_eq!(partyacc.get_name().as_str(),PARTYACC_NAME);
    }

    #[test]
    fn test_accountref_from_partyacc() {
        let mut partyacc = PartyAccount::default();

        partyacc.set_id(PARTYACC_ID);
        partyacc.set_name(PARTYACC_NAME);

        let accountref = AccountRef::from(partyacc.clone());

        assert_eq!(accountref.id,partyacc.get_id());
        assert_eq!(accountref.name,partyacc.get_name());
        assert_eq!(accountref.href,partyacc.get_href());

    }
}