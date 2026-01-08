use serde::{Serialize, Deserialize};
use super::{AttachmentRefOrValue, TaxDefinition};
use crate::common::extensible::Extensible;
use crate::TimePeriod;

/// Tax Exemption Certificate
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxExemptionCertificate {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Attachment entity and not the AttachmentRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<AttachmentRefOrValue>,
    ///Identifier of a document that shows proof of exemption from taxes for the taxing jurisdiction
    #[serde(rename = "certificateNumber")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate_number: Option<String>,
    ///Identifier of the tax exemption within list of the exemptions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Name of the jurisdiction that issued the exemption
    #[serde(rename = "issuingJurisdiction")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuing_jurisdiction: Option<String>,
    ///Reason for the tax exemption
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    ///A list of taxes that are covered by the exemption, e.g. City Tax, State Tax. The definition would include the exemption (e.g. for a rate exemption 0% would be a full exemption, 5% could be a partial exemption if the actual rate was 10%).
    #[serde(rename = "taxDefinition")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tax_definition: Vec<TaxDefinition>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for TaxExemptionCertificate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TaxExemptionCertificate {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for TaxExemptionCertificate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
