//! Party Role Module

use serde::{Deserialize, Serialize};

use crate::{HasId,HasName, CreateTMF,LIB_PATH, common::related_party::RelatedParty};
use tmflib_derive::{HasId,HasName};

use super::MOD_PATH;

const CLASS_PATH : &str = "partyRole";

/// Party Role
#[derive(Clone, Debug, Default, Deserialize, HasId, HasName, Serialize)]
pub struct PartyRole {
    /// Id of the Party Role
   pub id: Option<String>,
   /// HTML link
   pub href: Option<String>,
   /// Name of Role
   pub name: Option<String>,
   /// Entity that is associated with this role
   engaged_party: Option<RelatedParty>,
   /// Other related parties
   related_party: Vec<RelatedParty>,
}

impl PartyRole {
    /// Create new PartyRole with given name
    pub fn new(name : impl Into<String>) -> PartyRole {
        let mut role = PartyRole::create();
        role.name = Some(name.into());
        role
    }

    /// Set engaged party
    pub fn engaged_party(mut self, related_party: RelatedParty) -> PartyRole {
        self.engaged_party = Some(related_party);
        self
    }

    /// Add a related party
    pub fn add_party(&mut self, party : RelatedParty) {
        self.related_party.push(party);
    }
}