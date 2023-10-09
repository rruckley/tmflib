//! Individual Module

use serde::{Deserialize, Serialize};

use crate::{HasId,CreateTMF};
use crate::LIB_PATH;
use super::MOD_PATH;

const IND_PATH : &str = "individual";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Individual {
    id: Option<String>,
    href: Option<String>,
    full_name: String,
}

impl CreateTMF<Individual> for Individual {}

impl Individual {
    pub fn new(name : String) -> Individual {
        let mut ind = Individual::create();
        ind.full_name = name;
        ind
    }
}

impl HasId for Individual {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,IND_PATH,self.get_id());
        self.href = Some(href);    
    }
    fn generate_id(&mut self) {
        let id = self.get_uuid();
        self.id = Some(id);
        self.generate_href();    
    }
    fn get_href(&mut self) -> String {
        self.href.as_ref().unwrap().clone()   
    }
    fn get_id(&mut self) -> String {
        self.href.as_ref().unwrap().clone()    
    }
}