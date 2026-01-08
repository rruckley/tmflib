//! Characteristic Specification Module

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::common::tmf_error::TMFError;
use crate::{serde_value_to_type, Cardinality, TimePeriod};

use crate::HasDescription;
use tmflib_derive::HasDescription;

/// Characteristic Value Specification
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacteristicValueSpecification {
    /// Is this the default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Interval for a range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_interval: Option<String>,
    /// Pattern to match value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    /// Units for this value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    /// Validity time period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    /// Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    /// Start of value range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_from: Option<u32>,
    /// End of value range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_to: Option<u32>,
    /// Value Type, e.g. String, Integer etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

impl CharacteristicValueSpecification {
    /// Constructor
    pub fn new() -> CharacteristicValueSpecification {
        CharacteristicValueSpecification {
            is_default: Some(false),
            ..Default::default()
        }
    }

    /// Set regex for this characteristic value specification
    /// # Example
    /// ```
    /// # use tmflib::tmf633::characteristic_specification::CharacteristicValueSpecification;
    /// let cvs = CharacteristicValueSpecification::new()
    ///     .regex(String::from("[0-9]+(Mb|Gb)")).unwrap();
    /// ```
    pub fn regex(mut self, regex: String) -> Result<CharacteristicValueSpecification, TMFError> {
        let _re = Regex::new(&regex)?;
        self.regex = Some(regex);
        Ok(self)
    }

    /// Set value for this characteristic value specification
    /// # Example
    /// ```
    /// # use tmflib::tmf633::characteristic_specification::CharacteristicValueSpecification;
    /// # use serde_json::json;
    /// let cvs = CharacteristicValueSpecification::new()
    ///     .regex(String::from("[0-9]+(Mb|Gb)")).unwrap()
    ///     .value("100Mb".into()).unwrap();
    /// ```
    pub fn value(
        mut self,
        value: serde_json::Value,
    ) -> Result<CharacteristicValueSpecification, TMFError> {
        self.value_type = Some(serde_value_to_type(&value).to_string());
        match self.regex {
            Some(ref re_str) => {
                let re = Regex::new(re_str)?;
                let val_str = value.to_string();
                if !re.is_match(&val_str) {
                    return Err(TMFError::GenericError(format!(
                        "Value {} does not match regex {}",
                        val_str, re_str
                    )));
                }
                self.value = Some(value);
            }
            // If no regex, then just set the value
            None => self.value = Some(value),
        }
        Ok(self)
    }
}

/// Service Specification Characteristics
#[derive(Clone, Default, Debug, Deserialize, HasDescription, Serialize)]
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
    /// Characterisitc Value
    #[serde(skip_serializing_if = "Option::is_none")]
    characteristic_value_specification: Option<Vec<CharacteristicValueSpecification>>,
}

impl CharacteristicSpecification {
    /// Constructor
    pub fn new(name: impl Into<String>) -> CharacteristicSpecification {
        CharacteristicSpecification {
            name: Some(name.into()),
            max_cardinality: Some(1),
            value_type: Some("String".into()),
            is_unique: Some(false),
            ..Default::default()
        }
    }
    /// Set maximum cardinality
    pub fn cardinality(
        mut self,
        min_card: Cardinality,
        max_card: Cardinality,
    ) -> CharacteristicSpecification {
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
    pub fn description(mut self, description: impl Into<String>) -> CharacteristicSpecification {
        self.description = Some(description.into());
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const CHARSPEC_NAME: &str = "CharSpecName";
    const CARD_MIN: u16 = 7;
    const CARD_MAX: u16 = 8;
    const CHARSPEC_DESC: &str = "CharSpecDescription";

    const CHARSPEC_JSON: &str = "{
        \"name\" : \"CharacteristicSpecification\",
        \"id\" : \"CS123\",
        \"href\" : \"http://example.com/tmf633/spec/CS123\"
    }";

    #[test]
    fn test_charspec_cardinality() {
        let charspec =
            CharacteristicSpecification::new(CHARSPEC_NAME).cardinality(CARD_MIN, CARD_MAX);

        assert_eq!(charspec.min_cardinality, Some(CARD_MIN));
        assert_eq!(charspec.max_cardinality, Some(CARD_MAX));
    }

    #[test]
    fn test_charspec_optional() {
        let charspec = CharacteristicSpecification::new(CHARSPEC_NAME).optional();
        assert_eq!(charspec.min_cardinality, Some(0));
        assert_eq!(charspec.max_cardinality, Some(1));
    }

    #[test]
    fn test_charspec_mandatory() {
        let charspec = CharacteristicSpecification::new(CHARSPEC_NAME).mandatory();
        assert_eq!(charspec.min_cardinality, Some(1));
        assert_eq!(charspec.max_cardinality, Some(1));
    }

    #[test]
    fn test_charspec_description() {
        let charspec = CharacteristicSpecification::new(CHARSPEC_NAME).description(CHARSPEC_DESC);

        assert_eq!(charspec.description.is_some(), true);
        assert_eq!(charspec.description.unwrap().as_str(), CHARSPEC_DESC);
    }

    #[test]
    fn test_charspec_deserialization() {
        let charspec: CharacteristicSpecification = serde_json::from_str(CHARSPEC_JSON).unwrap();

        assert_eq!(charspec.id.is_some(), true);
        // assert_eq!(charspec.href.is_some(),true);
        assert_eq!(charspec.name.is_some(), true);

        assert_eq!(charspec.id.unwrap().as_str(), "CS123");
        // assert_eq!(charspec.href.unwrap().as_str(),"http://example.com/tmf633/spec/CS123");
        assert_eq!(
            charspec.name.unwrap().as_str(),
            "CharacteristicSpecification"
        );
    }

    #[test]
    fn test_charspec_regex() {
        let cvs = CharacteristicValueSpecification::new()
            .regex(String::from("[0-9]+(Mb|Gb)"))
            .unwrap()
            .value("100Mb".into())
            .unwrap();

        assert_eq!(cvs.regex.is_some(), true);
        assert_eq!(cvs.value.is_some(), true);
        assert_eq!(cvs.value_type.is_some(), true);
        assert_eq!(cvs.value_type.unwrap().as_str(), "String");
    }
}
