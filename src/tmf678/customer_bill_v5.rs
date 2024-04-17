//! Customer Bill Management v5
//! 

use super::MOD_PATH;
use serde::{Deserialize,Serialize};

use crate::{LIB_PATH,HasId,CreateTMF,DateTime,TimePeriod,HasLastUpdate,CreateTMFWithTime,Uri};
use tmflib_derive::{HasId,HasLastUpdate};
use crate::common::money::Money;
use crate::common::related_party::RelatedParty;
use crate::common::attachment::AttachmentRefOrValue;

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
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
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
    amount_due: Option<Money>,
    bill_date: Option<DateTime>,
    bill_no: String,
    billing_period: TimePeriod,
    category: String,
    /// Uri
    pub href: Option<Uri>,
    /// Unique Id
    pub id: Option<String>,
    /// Last update of bill
    pub last_update: Option<DateTime>,
    next_bill_date: DateTime,
    payment_due_date: DateTime,
    remaining_amount: Money,
    run_type: CustomerBillRunType,
    state: Option<CustomerBillStateType>,
    tax_excluded_amount: Money,
    tax_included_amount: Money,

    // Referenced Fields
    /// Related Parties
    pub related_party: Option<Vec<RelatedParty>>,
    /// Invoice / Bill documents
    pub bill_document: Option<Vec<AttachmentRefOrValue>>,
}

impl CustomerBill {
    /// Create a new customer bill
    pub fn new() -> CustomerBill {
        let mut bill = CustomerBill::create();
        bill.state = Some(CustomerBillStateType::default());
        bill
    }
}