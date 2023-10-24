//! Billing Account Module

use serde::{Deserialize, Serialize};

use crate::{HasId,HasLastUpdate,CreateTMF,CreateTMFWithTime, LIB_PATH};

use super::MOD_PATH;

const ACCOUNT_PATH : &str = "account";

/// Billing Account
#[derive( Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingAccount {
    id: Option<String>,
    href: Option<String>,
    name: String,
    last_update : Option<String>,
}

impl BillingAccount {
    /// Create new Billing Account
    pub fn new(name : &str) -> BillingAccount {
        let mut account = BillingAccount::create();
        account.name = name.to_owned();
        account
    }
}

impl HasId for BillingAccount {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,ACCOUNT_PATH,self.get_id());
        self.href = Some(href);    
    }
    fn generate_id(&mut self) {
        let id = BillingAccount::get_uuid();
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
        ACCOUNT_PATH.to_owned()
    }
}

impl CreateTMF<BillingAccount> for BillingAccount {}
impl HasLastUpdate for BillingAccount {
    fn set_last_update(&mut self, time : String) {
        self.last_update = Some(time);
    }
}
impl CreateTMFWithTime<BillingAccount> for BillingAccount {}