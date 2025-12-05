use serde::{Serialize, Deserialize};
use crate::common::extensible::Extensible;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeographicSubAddressUnit {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///The discriminator used for the subunit, often just a simple number but may also be a range.
    #[serde(rename = "subUnitNumber")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_unit_number: Option<String>,
    ///The type of subunit e.g.BERTH, FLAT, PIER, SUITE, SHOP, TOWER, UNIT, WHARF, RACK
    #[serde(rename = "subUnitType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_unit_type: Option<String>,
}
impl std::fmt::Display for GeographicSubAddressUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for GeographicSubAddressUnit {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for GeographicSubAddressUnit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
