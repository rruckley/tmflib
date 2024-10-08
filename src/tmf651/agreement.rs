//! Agreement Module

use serde::{Deserialize,Serialize};
use crate::{LIB_PATH,HasId,HasName, HasRelatedParty, HasDescription, TimePeriod, DateTime};
use tmflib_derive::{HasId,HasName,HasDescription};
use crate::common::related_party::RelatedParty;
use super::{agreement_item::AgreementItem, agreement_specification::AgreementSpecificationRef};
use crate::tmf648::quote::Quote;

use super::MOD_PATH;
const CLASS_PATH : &str = "agreement";

/// Agreeement / Contract
#[derive(Clone,Default,Debug, Deserialize, HasId, HasName, HasDescription, Serialize)]
pub struct Agreement {
    /// Period of this agreement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_period : Option<TimePeriod>,
    /// Type of this agreement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_type: Option<String>,
    /// Date for completion of agreement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<DateTime>,
    /// Detailed description
    pub description: Option<String>,
    /// Id of document
    pub document_number: u16,
    /// Start date
    pub initial_date: Option<DateTime>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id : Option<String>,
    /// URI for Agreement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Name of Agreement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Statement of Intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_of_intent: Option<String>,
    /// Agreement Version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Engaged Party for this Agreement
    pub engaged_party: Vec<RelatedParty>,
    /// Linked agreements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_agreement: Option<Vec<AgreementRef>>,
    /// Agreement Specifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_specification: Option<AgreementSpecificationRef>,
    /// Agreement Items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_item: Option<Vec<AgreementItem>>,
}

impl Agreement {
    /// Create a new Agreement
    pub fn new(name : impl Into<String>) -> Agreement {
        let mut agreement = Agreement::create();
        agreement.name = Some(name.into());
        agreement
    }
    /// Add a new item to the list
    pub fn add_item(&mut self, item : AgreementItem) {
        match self.agreement_item.as_mut() {
            Some(v) => {
                v.push(item);
            },
            None => {
                self.agreement_item = Some(vec![item]);
            }
        }
    }
}

/// Agreement Reference
#[derive(Clone,Default,Debug, Deserialize, Serialize)]
pub struct AgreementRef {
    /// Unique Id of referenced agreement
    id: String,
    /// URI of referenced agreement
    href: String,
    /// Name of referenced agreements
    name: String,
}

impl From<Agreement> for AgreementRef {
    fn from(value: Agreement) -> Self {
        AgreementRef {
            id: value.get_id(),
            href: value.get_href(),
            name: value.get_name(),
        }
    }
}

impl From<&Quote> for Agreement {
    fn from(value: &Quote) -> Self {
        let mut agreement = Agreement::new(format!("Agreement from: {}",value.get_name()));
        agreement.version.clone_from(&value.version);
        agreement.agreement_period = Some(TimePeriod::period_days(365));
        agreement.description.clone_from(&value.description);
        let party = value.get_party(0);
        if party.is_some() {
            agreement.engaged_party = vec![party.cloned().unwrap()];
        }
        // Iterate through 
        if value.quote_item.is_some() {
            let items = value.quote_item.as_ref().unwrap();
            items.iter().for_each(|i| {
                // Take each QuoteItem and convert to AgreementItem
                let agreement_item = AgreementItem::from(i);
                agreement.add_item(agreement_item);
            })
        }
        agreement
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const AGREEMENT : &str = "AnAgreement";
    const QUOTE_VERS: &str = "1.0";
    const QUOTE_DESC: &str = "QuoteDescription";

    #[test]
    fn test_agreement_new_name() {
        let agreement = Agreement::new(AGREEMENT);

        assert_eq!(agreement.name,Some(AGREEMENT.into()));
    }

    #[test]
    fn test_agreement_ref_from() {
        let agreement = Agreement::new(AGREEMENT);
        let agreement_ref = AgreementRef::from(agreement.clone());

        assert_eq!(agreement.id,Some(agreement_ref.id));
        assert_eq!(agreement.name,Some(agreement_ref.name));
        assert_eq!(agreement.href,Some(agreement_ref.href)); 
    }

    #[test]
    fn test_agreement_from_quote() {
        let mut quote = Quote::new();
        quote.version = Some(QUOTE_VERS.to_string());
        quote.description = Some(QUOTE_DESC.to_string());
        let agreement = Agreement::from(&quote);

        assert_eq!(quote.version,agreement.version);
        assert_eq!(agreement.agreement_period.is_some(),true);
        assert_eq!(agreement.description.is_some(),true);
        assert_eq!(agreement.description.unwrap().as_str(),QUOTE_DESC);
    }

    #[test]
    fn test_agreement_add_item() {
        let item1 = AgreementItem::default();
        let mut agreement = Agreement::new(AGREEMENT);

        agreement.add_item(item1);

        let item2 = AgreementItem::default();

        agreement.add_item(item2);

        assert_eq!(agreement.agreement_item.is_some(),true);
        assert_eq!(agreement.agreement_item.unwrap().len(),2);
    }
}