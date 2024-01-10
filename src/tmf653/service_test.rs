//! Service Test

use serde::{Deserialize,Serialize};

use crate::{HasId,CreateTMF,TimePeriod,LIB_PATH};
use crate::common::related_party::RelatedParty;
use super::MOD_PATH;

const TEST_PATH : &str = "serviceTest";

/// Test execution status
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub enum ExecutionStateType {
    /// Acknowledged
    #[default]
    Acknowledged,
    /// In Progress
    InProgress,
    /// Rejected
    Rejected,
    /// Pending
    Pending,
    /// Cancelled
    Cancelled,
    /// Completed
    Completed,
    /// Failed
    Failed,
}

/// Service Test
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct ServiceTest {
    description: Option<String>,
    end_date_time: Option<String>,
    href: Option<String>,
    id: Option<String>,
    mode: Option<String>,
    name: Option<String>,
    start_date_time: Option<String>,
    state: Option<ExecutionStateType>,
    valid_for: Option<TimePeriod>,
    related_party: Option<Vec<RelatedParty>>,
}

impl ServiceTest {
    /// Create new ServiceTest
    pub fn new(name : &str) -> ServiceTest {
        let mut st = ServiceTest::create();
        st.name = Some(name.to_owned());
        st
    }
}

impl HasId for ServiceTest {
    fn generate_href(&mut self) {
        let href = format!("{}/{}",ServiceTest::get_class_href(),self.get_id());
        self.href = Some(href);        
    }
    fn generate_id(&mut self) {
        let id = ServiceTest::get_uuid();
        self.id = Some(id);  
        self.generate_href();
    }
    fn get_class() -> String {
        String::from("service_test")    
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()
    }
    fn get_class_href() -> String {
        format!("/{}/{}/{}",LIB_PATH,MOD_PATH,TEST_PATH)    
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()    
    }
}

impl CreateTMF<ServiceTest> for ServiceTest {}