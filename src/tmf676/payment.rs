//! TMF676 Payment Module
//!

use crate::common::money::Money;
use crate::common::related_entity::EntityRef;
use crate::common::related_party::RelatedParty;
use crate::tmf666::AccountRef;
use crate::tmf676::PaymentMethodRefOrValue;
use crate::{DateTime, HasDescription, HasId, HasName, Uri,vec_insert};
use serde::{Deserialize, Serialize};

use tmflib_derive::{HasDescription, HasId, HasName};

use super::MOD_PATH;

const CLASS_PATH: &str = "payment";

/// Reference to another TMF schema
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct PaymentItem {
    amount : Money,
    id  : String,
    tax_amount: Money,
    total_amount: Money,
    item: EntityRef,
}

/// A Payment
#[derive(Clone, Debug, Default, HasId, HasName, HasDescription, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    /// Payment Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_id: Option<String>,
    /// Payment Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_amount: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_amount: Option<Money>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTTP Uri
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Refund Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // Referenced Objects
    payer : Option<RelatedParty>,
    payment_method: PaymentMethodRefOrValue,
    account: AccountRef,
    payment_item: Option<Vec<PaymentItem>>,
}

impl Payment {
    /// Create a new Payment from a payment method and account
    pub fn new(method: PaymentMethodRefOrValue, account : AccountRef) -> Payment {
        Payment {
            account,
            payment_method: method,
            ..Default::default()
        }
    }

    /// Set the payer
    pub fn payer(mut self, party : impl Into<RelatedParty>) -> Payment {
        self.payer = Some(party.into());
        self
    }

    /// Add paymet item to the payment
    pub fn item(mut self, item: PaymentItem) -> Payment {
        vec_insert(&mut self.payment_item,item);
        self
    }
}
