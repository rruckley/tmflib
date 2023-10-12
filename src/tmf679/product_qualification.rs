//! Product Qualificaiton Module


use serde::{Deserialize,Serialize};
use uuid::Uuid;

use crate::HasId;
use crate::CreateTMF;
use crate::LIB_PATH;

use super::MOD_PATH;
use crate::common::related_party::RelatedParty;

const POQ_PATH : &str = "qualification";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum TaskStateType {
    Acknowledged,
    TerminatedWithError,
    #[default]
    InProgress,
    Done,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProductOfferingQualification {
    id: Option<String>,
    href: Option<String>,
    state: TaskStateType,
    related_party: Vec<RelatedParty>,
}

impl HasId for ProductOfferingQualification {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,POQ_PATH,self.get_id());   
        self.href = Some(href);  
    }
    fn generate_id(&mut self) {
        let id = Uuid::new_v4().simple().to_string();
        self.id = Some(id);
    }
    fn get_href(&mut self) -> String {
        self.href.as_ref().unwrap().clone()    
    }
    fn get_id(&mut self) -> String {
        self.id.as_ref().unwrap().clone()    
    }
}

impl CreateTMF<ProductOfferingQualification> for ProductOfferingQualification {}

impl ProductOfferingQualification {
    pub fn new() -> ProductOfferingQualification {
        ProductOfferingQualification::create()
    }
    pub fn add_party(&mut self, party : RelatedParty) {
        self.related_party.push(party);
    }
}

