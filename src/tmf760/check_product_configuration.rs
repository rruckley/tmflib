//! Check Product Configuration Module
//! 

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{HasId,CreateTMF};
use crate::LIB_PATH;
use super::MOD_PATH;
const CPC_PATH : &str = "CheckProductConfiguration";


/// Configuration Check Status
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskStateType {
    /// Config request has been recieved and acknowledged
    #[default]
    Acknowledged,
    /// Config request is in progress
    InProgress,
    /// Config request has been rejected
    Rejected,
    /// Config request has been rejected with an error
    TerminatedWithError,
    /// Config request has been cancelled
    Cancelled,
    /// Config request has completed
    Done,
}

/// Check Product Configuration request object
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckProductConfiguration {
    id: Option<String>,
    href: Option<String>,
    instant_sync: bool,
    provide_alternatives: bool,
    state: TaskStateType,
}

impl CheckProductConfiguration {
    /// Create a product configuration check request
    pub fn new() -> CheckProductConfiguration {
        CheckProductConfiguration::create()
    }
}

impl CreateTMF<CheckProductConfiguration> for CheckProductConfiguration {}

impl HasId for CheckProductConfiguration {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,CPC_PATH,self.get_id());
        self.href = Some(href);  
    }
    fn generate_id(&mut self) {
        let id = Uuid::new_v4().simple().to_string();
        self.id = Some(id);
        self.generate_href();    
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()    
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()     
    }
    fn get_class() -> String {
        CPC_PATH.to_owned()
    }
    
}