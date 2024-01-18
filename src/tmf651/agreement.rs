//! Agreement Module

use serde::{Deserialize,Serialize};
use crate::{LIB_PATH,HasId,HasName,CreateTMF,TimePeriod};
use tmflib_derive::{HasId,HasName};
use crate::common::related_party::RelatedParty;
use super::agreement_specification::AgreementSpecificationRef;

use super::MOD_PATH;
const CLASS_PATH : &str = "agreement";

/// Agreeement / Contract
#[derive(Clone,Default,Debug, Deserialize, HasId, HasName, Serialize)]
pub struct Agreement {
    /// Period of this agreement
    pub agreement_period : Option<TimePeriod>,
    /// Type of this agreement
    pub agreement_type: Option<String>,
    /// Date for completion of agreement
    pub completion_date: Option<TimePeriod>,
    /// Detailed description
    pub description: String,
    /// Id of document
    pub document_number: u16,
    /// Start date
    pub initial_date: String,
    /// Unique Id
    pub id : Option<String>,
    /// URI for Agreement
    pub href: Option<String>,
    /// Name of Agreement
    pub name: Option<String>,
    /// Statement of Intent
    pub statement_of_intent: Option<String>,
    /// Agreement Version
    pub version: Option<String>,
    /// Engaged Party for this Agreement
    pub engaged_party: Vec<RelatedParty>,
    /// Linked agreements
    pub associated_agreement: Option<Vec<AgreementRef>>,
    pub agreement_specification: Option<AgreementSpecificationRef>,
}

impl Agreement {
    /// Create a new Agreement
    pub fn new(name : impl Into<String>) -> Agreement {
        let mut agreement = Agreement::create();
        agreement.name = Some(name.into());
        agreement
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