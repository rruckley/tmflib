//! Work Module

use crate::{
    LIB_PATH,
    HasId,
    HasName,
    TimePeriod,
    Uri,
};
use super::MOD_PATH;
use tmflib_derive::{
    HasId,
    HasName,
};
use serde::{Deserialize,Serialize};
use crate::tmf646::appointment::AppointmentRef;
use crate::tmf651::agreement::AgreementRef;

const CLASS_PATH : &str = "work";

/// Reference to a work object
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct WorkRef {
    /// HREF to object
    pub href: Uri,
    /// Unique Id for object
    pub id: String,
    /// Name
    pub name: String,
}

impl From<Work> for WorkRef {
    fn from(value: Work) -> Self {
        WorkRef {
            href: value.get_href(),
            id : value.get_id(),
            name: value.get_name(),
        }
    }
}

/// Work Value or Reference.
#[derive(Clone,Debug,Deserialize,Serialize)]
#[serde(untagged)]
pub enum WorkRefOrValue {
    /// Reference variant
    Ref(WorkRef),
    /// Value variant
    Val(Work),
}

impl From<WorkRef> for WorkRefOrValue {
    fn from(value: WorkRef) -> Self {
        WorkRefOrValue::Ref(value)
    }
}

impl From<Work> for WorkRefOrValue {
    fn from(value: Work) -> Self {
        WorkRefOrValue::Val(value)
    }
}

/// Work
#[derive(Clone,Debug,Default,Deserialize,HasId,HasName,Serialize)]
pub struct Work {
    /// Metadata: Schema Type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    /// Metadata: Base Type for derived types
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    pub base_type: Option<String>,
    /// Metadata: Schema of derived classes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@schemaLocation")]
    pub schema_location: Option<String>,
    /// HREF to object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Unique Id for object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Date when the order was completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>, 
    /// When can the work be delivered?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_time_slot: Option<TimePeriod>,
    /// Description of the work
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Date when the requester expects the work to be completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_completion_date: Option<String>, 
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // Referenced Types
    /// Associated agreements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement: Option<Vec<AgreementRef>>,
    /// Appointment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointment : Option<AppointmentRef>,
}

impl Work {
    /// Create a new work object
    pub fn new(name : impl Into<String>) -> Work {
        let mut out = Work::create();
        out.set_name(name);
        out
    }
}