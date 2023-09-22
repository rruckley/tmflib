//! TMF629 Customer Management Module
//! 
//! 
//! 

use super::customer::Customer;

use serde::{Deserialize,Serialize};

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct TMF629CustomerManagement {
    customers : Vec<Customer>,
}

impl TMF629CustomerManagement {
    pub fn new() -> TMF629CustomerManagement {
        TMF629CustomerManagement { 
            customers: vec![] 
        }
    }
}