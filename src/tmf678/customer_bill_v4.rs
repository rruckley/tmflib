//! Customer Bill Management V4
//! 

use super::MOD_PATH;
use serde::{Deserialize,Serialize};

use crate::{
    LIB_PATH,
    HasId, 
    HasAttachment,
    HasLastUpdate,
    DateTime,
    TimePeriod,
    Uri
};
use tmflib_derive::{
    HasId,
    HasLastUpdate
};
use crate::tmf666::billing_account::BillingAccountRef;
use crate::common::money::Money;
use crate::common::related_party::RelatedParty;
use crate::common::attachment::AttachmentRefOrValue;
use crate::common::tax_item::TaxItem;

const CLASS_PATH : &str = "customer_bill";

/// Customer Bill Run Type
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub enum CustomerBillRunType {
    /// Inside regular bill cycle
    #[default]
    OnCycle,
    /// Outside regular bill cycle
    OffCycle,
}

/// Customer Bill Status
#[derive(Clone,Debug,Default,Deserialize,PartialEq, Serialize)]
pub enum CustomerBillStateType {
    /// New Bill
    #[default]
    New,
    /// Bill has been paused
    OnHold,
    /// Bill has passed validation
    Validated,
    /// Bill has been sent to customer
    Sent,
    /// Bill has been paid
    Settled,
    /// Bill has been partially paid
    PartialPaid,
}

/// Customer Bill
#[derive(Clone,Debug,Default,Deserialize,HasId,HasLastUpdate,Serialize)]
pub struct CustomerBill {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_due: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bill_date: Option<DateTime>,
    bill_no: String,
    billing_period: TimePeriod,
    category: String,
    /// Uri
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Last update of bill
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<DateTime>,
    next_bill_date: DateTime,
    payment_due_date: DateTime,
    remaining_amount: Money,
    run_type: CustomerBillRunType,
    /// Customer Bill Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CustomerBillStateType>,
    tax_excluded_amount: Money,
    tax_included_amount: Money,

    // Referenced Fields
    /// Related Parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// Invoice / Bill documents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_document: Option<Vec<AttachmentRefOrValue>>,
    /// Billing Account References
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_account : Option<Vec<BillingAccountRef>>,
    /// Tax Items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_item: Option<Vec<TaxItem>>,
}

impl CustomerBill {
    /// Create a new customer bill
    pub fn new() -> CustomerBill {
        let mut bill = CustomerBill::create();
        bill.state = Some(CustomerBillStateType::default());
        bill
    }
}

impl HasAttachment for CustomerBill {
    fn add(&mut self, attachment : &AttachmentRefOrValue) {
        match self.bill_document.as_mut() {
            Some(v) => {
                v.push(attachment.clone());
            }
            None => {
                self.bill_document = Some(vec![attachment.clone()]);
            }
        }    
    }
    fn position(&self, name : impl Into<String>) -> Option<usize> {
        match self.bill_document.as_ref() {
            Some(v) => {
                let pattern : String = name.into();
                v.iter().position(|a| a.name == Some(pattern.clone()))
            }
            None => None,
        }
    }
    fn find(&self, name : impl Into<String>) -> Option<&AttachmentRefOrValue> {
        match self.bill_document.as_ref() {
            Some(v) => {
                let pattern : String = name.into();
                v.iter().find(|a| a.name == Some(pattern.clone()))               
            },
            None => None,
        }
    }
    fn get(&self, position: usize) -> Option<AttachmentRefOrValue> {
        match self.bill_document.as_ref() {
            Some(v) => {
                v.get(position).cloned()
            },
            None => None,
        }    
    }
    fn remove(&mut self, position : usize) -> Option<AttachmentRefOrValue> {
        self.bill_document.as_mut().map(|v| v.remove(position))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_customer_bill_new_state() {
        let bill = CustomerBill::new();

        assert_eq!(bill.state,Some(CustomerBillStateType::default()));
    }
}