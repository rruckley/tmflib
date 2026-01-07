use crate::common::extensible::ExtensibleFvo;
use crate::TimePeriod;
use serde::{Deserialize, Serialize};

/// CharacteristicValueSpecification defined in terms of a base schema that is extended to add attributes specific to the kind of characteristic value specification being described
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacteristicValueSpecificationFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///If true, the Boolean Indicates if the value is the default value for a characteristic
    #[serde(rename = "isDefault")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    ///An indicator that specifies the inclusion or exclusion of the valueFrom and valueTo attributes. If applicable, possible values are "open", "closed", "closedBottom" and "closedTop".
    #[serde(rename = "rangeInterval")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_interval: Option<String>,
    ///A regular expression constraint for given value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    ///A length, surface, volume, dry measure, liquid measure, money, weight, time, and the like. In general, a determinate quantity or magnitude of the kind designated, taken as a standard of comparison for others of the same kind, in assigning to them numerical values, as 1 foot, 1 yard, 1 mile, 1 square foot.
    #[serde(rename = "unitOfMeasure")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///The low range value that a characteristic can take on
    #[serde(rename = "valueFrom")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_from: Option<i64>,
    ///The upper range value that a characteristic can take on
    #[serde(rename = "valueTo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_to: Option<i64>,
    ///A kind of value that the characteristic value can take on, such as numeric, text and so forth
    #[serde(rename = "valueType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}
impl std::fmt::Display for CharacteristicValueSpecificationFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CharacteristicValueSpecificationFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for CharacteristicValueSpecificationFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
