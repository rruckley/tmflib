//! Shipping Instruction Module

use crate::{
    TimePeriod,
    HasId, 
    Uri,
    LIB_PATH,
    HasNote,
};
use crate::common::money::Money;
use crate::common::note::Note;
use crate::common::tmf_error::TMFError;
use tmflib_derive::{HasId,HasNote};
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
#[derive(Clone,Default,Debug,Deserialize,HasId,HasNote,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShippingInstruction {
    carrier_id: String,
    carrier_name: String,
    carrier_service_code: String,
    delivery_attempts: u16,
    delivery_speed: String,
    delivery_time_slot: TimePeriod,
    /// Uri for instruction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Unique Id for instruction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insured_value: Option<Money>,
    /// Message for the shipping label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_message: Option<String>,
    package_type: String,
    receipt_confirmation: String,
    shipping_type: String,
    signature_required: bool,
    signature_required_by : Option<SignatureRequiredByType>,
    warehouse_id : String,
    // Referenced Struct
    /// Notes
    pub note : Option<Vec<Note>>,
}

impl ShippingInstruction {
    /// Create a new shipping instruction
    pub fn new(instruction : impl Into<String>) -> ShippingInstruction {
        ShippingInstruction::create()
            .message(instruction)
    }

    /// Set the label message for this instructions
    pub fn message(mut self, message : impl Into<String>) -> ShippingInstruction {
        self.label_message = Some(message.into());
        self
    }

    /// Set the signature requirements
    pub fn signature_required_by(mut self, signature : Option<SignatureRequiredByType>) -> ShippingInstruction {
        match signature {
            Some(s) => { 
                self.signature_required_by = Some(s);
                self.signature_required = true;
            },
            None => {
                self.signature_required_by = None;
                self.signature_required = false;
            },
        };
        self
    }
}

impl From<String> for ShippingInstruction {
    fn from(value: String) -> Self {
        ShippingInstruction::new(value)
    }
}

#[cfg(test)]
mod test {
    use super::{ShippingInstruction, SignatureRequiredByType};
    const INST : &str = "AnInstruction";

    #[test]
    fn test_instruction_create() {
        let instruction = ShippingInstruction::new(INST);

        assert_eq!(instruction.label_message.unwrap(),INST.to_string());
    }

    #[test]
    fn test_instruction_from_string() {
        let instruction : ShippingInstruction = INST.to_string().into();

        assert_eq!(instruction.label_message.unwrap(),INST.to_string());
    }

    #[test]
    fn test_instruction_signature() {
        let instruction = ShippingInstruction::new(INST)
            .signature_required_by(Some(SignatureRequiredByType::Receiver));

        assert_eq!(instruction.signature_required,true);
    }
}