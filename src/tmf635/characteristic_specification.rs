use super::{CharacteristicSpecificationRelationship, CharacteristicValueSpecification};
use crate::TimePeriod;
use serde::{Deserialize, Serialize};
///This class defines a characteristic specification.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacteristicSpecification {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///This (optional) field provides a link to the schema describing the value type.
    #[serde(rename = "@valueSchemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_schema_location: Option<String>,
    ///An aggregation, migration, substitution, dependency or exclusivity relationship between/among Specification Characteristics.
    #[serde(rename = "charSpecRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub char_spec_relationship: Option<Vec<CharacteristicSpecificationRelationship>>,
    ///A CharacteristicValueSpecification object is used to define a set of attributes, each of which can be assigned to a corresponding set of attributes in a CharacteristicSpecification object. The values of the attributes in the CharacteristicValueSpecification object describe the values of the attributes that a corresponding Characteristic object can take on.
    #[serde(rename = "characteristicValueSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characteristic_value_specification: Option<Vec<CharacteristicValueSpecification>>,
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
    ///An indicator that specifies if a value is unique for the specification. Possible values are; "unique while value is in effect" and "unique whether value is in effect or not"
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///A rule or principle represented in regular expression used to derive the value of a characteristic value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///A kind of value that the characteristic can take on, such as numeric, text and so forth
    #[serde(rename = "valueType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}
impl std::fmt::Display for CharacteristicSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
