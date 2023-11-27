//! Cost Module

use crate::{HasId,CreateTMF,LIB_PATH};
use serde::{Deserialize,Serialize};

use super::MOD_PATH;

const COST_PATH : &str = "cost";

/// Cost Management
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    /// Unique Id
    pub id: Option<String>,
    /// HREF to this object
    pub href: Option<String>,
    /// Name of this cost
    pub name: Option<String>,
}

impl Cost {
    /// Create new cost entry
    pub fn new(name : &str) -> Cost {
        let mut cost = Cost::create();
        cost.name = Some(name.to_owned());
        cost
    }
}

impl HasId for Cost {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,COST_PATH,self.get_id());
        self.href = Some(href);    
    }
    fn generate_id(&mut self) {
        let id = Cost::get_uuid();
        self.id = Some(id);
        self.generate_href();    
    }
    fn get_class() -> String {
        COST_PATH.to_string()    
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()    
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()       
    }
}

impl CreateTMF<Cost> for Cost {}