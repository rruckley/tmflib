//! Shipping Instruction Module

use crate::{TimePeriod,HasId,CreateTMF,Uri,LIB_PATH};
use crate::common::money::Money;
use tmflib_derive::HasId;
use serde::{Deserialize,Serialize};

use super::MOD_PATH;
const CLASS_PATH : &str = "instruction";

/// Signarure Required Type
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub enum SignatureRequiredByType {
    /// Signature required by an Adult
    Adult,
    /// Signature required by the reciever
    #[default]
    Receiver,
}

/// Shipping Item Instructions
#[derive(Clone,Default,Debug,Deserialize,HasId,Serialize)]
pub struct ShippingInstruction {
    carrier_id: String,
    carrier_name: String,
    carrier_service_code: String,
    delivery_attempts: u16,
    delivery_speed: String,
    delivery_time_slot: TimePeriod,
    /// Uri for instruction
    pub href: Option<Uri>,
    /// Unique Id for instruction
    pub id: Option<String>,
    insured_value: Option<Money>,
    label_message: Option<String>,
    package_type: String,
    receipt_confirmation: String,
    shipping_type: String,
    signature_required: bool,
    signature_required_by : SignatureRequiredByType,
    warehouse_id : String,
}

impl ShippingInstruction {
    /// Create a new shipping instruction
    pub fn new(instruction : impl Into<String>) -> ShippingInstruction {
        ShippingInstruction::create()
            .message(instruction)
    }

    fn message(mut self, message : impl Into<String>) -> ShippingInstruction {
        self.label_message = Some(message.into());
        self
    }
}