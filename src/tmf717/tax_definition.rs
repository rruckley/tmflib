use serde::{Serialize, Deserialize};
use crate::TimePeriod;
use crate::common::extensible::Extensible;

/// Tax Definition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxDefinition {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Unique identifier of the tax.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Level of the jurisdiction that levies the tax
    #[serde(rename = "jurisdictionLevel")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction_level: Option<String>,
    ///Name of the jurisdiction that levies the tax
    #[serde(rename = "jurisdictionName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction_name: Option<String>,
    ///Tax name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Type of the tax.
    #[serde(rename = "taxType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for TaxDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TaxDefinition {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for TaxDefinition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
