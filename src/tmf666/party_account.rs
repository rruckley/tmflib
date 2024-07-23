//! Party Account Module

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
use super::AccountBalance;
use super::AccountRef;

use super::MOD_PATH;
const CLASS_PATH : &str = "account";

/// Party Account
#[derive( Clone, Debug, Default, Deserialize, HasId, HasName, HasLastUpdate, Serialize)]
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
    related_party: Vec<RelatedParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<Vec<Contact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_balance: Option<Vec<AccountBalance>>,
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