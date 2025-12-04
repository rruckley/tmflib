use serde::{Serialize, Deserialize};
use super::{
    CharacteristicSpecificationRelationshipFvo, CharacteristicValueSpecificationFvo,
    ExtensibleFvo,
};
use crate::TimePeriod;

/// A specification that defines a characteristic that can be used to describe an entity.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacteristicSpecificationFvo {
    ///This (optional) field provides a link to the schema describing the value type.
    #[serde(rename = "@valueSchemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_schema_location: Option<String>,
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///An aggregation, migration, substitution, dependency or exclusivity relationship between/among Specification Characteristics.
    #[serde(rename = "charSpecRelationship")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub char_spec_relationship: Vec<CharacteristicSpecificationRelationshipFvo>,
    ///A CharacteristicValueSpecification object is used to define a set of attributes, each of which can be assigned to a corresponding set of attributes in a CharacteristicSpecification object. The values of the attributes in the CharacteristicValueSpecification object describe the values of the attributes that a corresponding Characteristic object can take on.
    #[serde(rename = "characteristicValueSpecification")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic_value_specification: Vec<CharacteristicValueSpecificationFvo>,
    ///If true, the Boolean indicates that the target Characteristic is configurable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configurable: Option<bool>,
    ///A narrative that explains the CharacteristicSpecification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///An indicator that specifies that the values for the characteristic can be extended by adding new values when instantiating a characteristic for a resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensible: Option<bool>,
    ///Unique ID for the characteristic
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Specifies if the value of this characteristic is unique across all entities instantiated from the specification that uses this characteristc. For example, consider a ProductSpecification for a set-top box, with a CharacteristicSpecification cardID. Each set-top box must have a different value for cardID, so this isUnique attribute would be set to true for the characteristic.
    #[serde(rename = "isUnique")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
    ///The maximum number of instances a CharacteristicValue can take on. For example, zero to five phone numbers in a group calling plan, where five is the value for the maxCardinality.
    #[serde(rename = "maxCardinality")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_cardinality: Option<i64>,
    ///The minimum number of instances a CharacteristicValue can take on. For example, zero to five phone numbers in a group calling plan, where zero is the value for the minCardinality.
    #[serde(rename = "minCardinality")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_cardinality: Option<i64>,
    ///A word, term, or phrase by which this characteristic specification is known and distinguished from other characteristic specifications.
    pub name: String,
    ///A rule or principle represented in regular expression used to derive the value of a characteristic value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///A kind of value that the characteristic can take on, such as numeric, text and so forth
    #[serde(rename = "valueType")]
    pub value_type: String,
}
impl std::fmt::Display for CharacteristicSpecificationFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CharacteristicSpecificationFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for CharacteristicSpecificationFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
