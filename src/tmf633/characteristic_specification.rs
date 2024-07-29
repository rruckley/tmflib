//! Characteristic Specification Module

use serde::{Deserialize,Serialize};

use crate::{Cardinality, TimePeriod};

/// Service Specification Characteristics
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct CharacteristicSpecification {
    /// Can this characteristic be configured?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurable: Option<bool>,
    /// Details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Can this characteristic be extended?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensible: Option<bool>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Is there a unique constraint on the value of this characteristic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Minimum instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_cardinality: Option<Cardinality>,
    /// Maximum instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cardinality: Option<Cardinality>,
    /// Validity Period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    /// Type for this characteristic, e.g. String, Integer etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

impl CharacteristicSpecification {
    /// Constructor
    pub fn new(name : impl Into<String>) -> CharacteristicSpecification {
        CharacteristicSpecification {
            name : Some(name.into()),
            max_cardinality: Some(1),
            value_type: Some("String".into()),
            is_unique : Some(false),
            ..Default::default()
        }
    }
    /// Set maximum cardinality
    pub fn cardinality(mut self, min_card : Cardinality, max_card : Cardinality) -> CharacteristicSpecification {
        self.min_cardinality = Some(min_card);
        self.max_cardinality = Some(max_card);
        self
    }

    /// Set characteristic as optional cardinality => (0..1)
    pub fn optional(mut self) -> CharacteristicSpecification {
        self.min_cardinality = Some(0);
        self.max_cardinality = Some(1);
        self
    }

    /// Make this characteristic mandatory
    pub fn mandatory(mut self) -> CharacteristicSpecification {
        self.min_cardinality = Some(1);
        self.max_cardinality = Some(1);
        self
    }

    /// Set the description of this characteristic
    pub fn description(mut self,description : impl Into<String>) -> CharacteristicSpecification {
        self.description = Some(description.into());
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const CHARSPEC_NAME : &str = "CharSpecName";
    const CARD_MIN : u16 = 7;
    const CARD_MAX : u16 = 8;
    const CHARSPEC_DESC : &str = "CharSpecDescription";

    const CHARSPEC_JSON : &str = "{
        \"name\" : \"CharacteristicSpecification\",
        \"id\" : \"CS123\",
        \"href\" : \"http://example.com/tmf633/spec/CS123\"
    }";

    #[test]
    fn test_charspec_cardinality() {
        let charspec = CharacteristicSpecification::new(CHARSPEC_NAME)
            .cardinality(CARD_MIN,CARD_MAX);

        assert_eq!(charspec.min_cardinality,Some(CARD_MIN));
        assert_eq!(charspec.max_cardinality,Some(CARD_MAX));    
    }

    #[test]
    fn test_charspec_optional() {
        let charspec = CharacteristicSpecification::new(CHARSPEC_NAME)
            .optional();
        assert_eq!(charspec.min_cardinality,Some(0));
        assert_eq!(charspec.max_cardinality,Some(1));
    }

    #[test]
    fn test_charspec_mandatory() {
        let charspec = CharacteristicSpecification::new(CHARSPEC_NAME)
            .mandatory();
        assert_eq!(charspec.min_cardinality,Some(1));
        assert_eq!(charspec.max_cardinality,Some(1));
    }

    #[test]
    fn test_charspec_description() {
        let charspec = CharacteristicSpecification::new(CHARSPEC_NAME)
        .description(CHARSPEC_DESC);

        assert_eq!(charspec.description.is_some(),true);
        assert_eq!(charspec.description.unwrap().as_str(),CHARSPEC_DESC);
    }

    #[test]
    fn test_charspec_deserialization() {
        let charspec : CharacteristicSpecification = serde_json::from_str(CHARSPEC_JSON).unwrap();

        assert_eq!(charspec.id.is_some(),true);
        // assert_eq!(charspec.href.is_some(),true);
        assert_eq!(charspec.name.is_some(),true);

        assert_eq!(charspec.id.unwrap().as_str(),"CS123");
        // assert_eq!(charspec.href.unwrap().as_str(),"http://example.com/tmf633/spec/CS123");
        assert_eq!(charspec.name.unwrap().as_str(),"CharacteristicSpecification");
    }
}