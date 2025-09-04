//! Shipment Specification Module

use crate::common::attachment::AttachmentRefOrValue;
use regex::Regex;
use crate::common::tmf_error::TMFError;
use serde::{Deserialize, Serialize};
use tmflib_derive::{HasAttachment, HasDescription, HasId, HasLastUpdate, HasName, HasValidity};

use crate::{
    DateTime, HasAttachment, HasDescription, HasId, HasLastUpdate, HasName, HasValidity,
    TimePeriod, Uri,
    serde_value_to_type,
};

use super::MOD_PATH;
const CLASS_PATH: &str = "shippingSpecification";

/// Shipment Specification
#[derive(
    Clone,
    Default,
    Debug,
    Deserialize,
    HasId,
    HasAttachment,
    HasName,
    HasLastUpdate,
    HasValidity,
    HasDescription,
    Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentSpecificationRefOrValue {
    /// Description
    pub description: Option<String>,
    /// HTTP Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Is this a bundle of specifications?
    pub is_bundle: bool,
    /// Last time updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<DateTime>,
    /// Status
    pub lifecycle_status: String,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Validity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    /// Version
    pub version: String,
    // Referenced Structs
    shipment_spec_relationship: Option<Vec<ShipmentSpecificationRelationship>>,
    /// Shipment Specification Characteristic
    shipment_specification_characteristic: Option<Vec<CharacteristicSpecification>>,
    /// Attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentRefOrValue>>,
}

/// Shipment Specification Relationship
#[derive(Clone, Default, Debug, HasId, HasName, HasValidity, Deserialize, Serialize)]
pub struct ShipmentSpecificationRelationship {
    /// HTTP Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Specification Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Relationship Type
    #[serde(skip_serializing_if = "Option::is_none")]
    relationship_type: Option<String>,
    /// Role
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
    /// Validity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}

/// Shipping Characteristic Specification
#[derive(Clone, Default, Debug, HasValidity, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacteristicSpecification {
    configurable: bool,
    description: String,
    extensible: bool,
    /// Unique Id
    pub id: Option<String>,
    is_unique: bool,
    max_cardinality: u16,
    min_cardinality: u16,
    name: Option<String>,
    regex: Option<String>,
    valid_for: Option<TimePeriod>,
    value_type: String,
    // Referenced Struct
    characteristic_value_specification: Option<Vec<CharacteristicValueSpecification>>,
    char_spec_relationship: Option<Vec<CharacteristicSpecificationRelationship>>,
}

/// Shipping Order Characteristic Speficiation
#[derive(Clone, Default, Debug, HasValidity, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacteristicSpecificationRelationship {
    characteristic_specification_id: String,
    name: String,
    parent_specification_href: Uri,
    parent_specification_id: String,
    relationship_type: String,
    valid_for: Option<TimePeriod>,
}

/// Shipping Characteristic Value Specification
#[derive(Clone, Default, Debug, HasValidity, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacteristicValueSpecification {
    /// Is this the default value
    #[serde(skip_serializing_if = "Option::is_none")]
    is_default: Option<bool>,
    range_interval: String,
     /// Pattern to match value
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<String>,
    unit_of_measure: String,
    /// Validity
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    /// Value
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<serde_json::Value>,
    value_from: u16,
    value_to: u16,
    /// Value Type, e.g. String, Integer etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    value_type: Option<String>,
}

impl CharacteristicValueSpecification {
    /// Constructor
    pub fn new() -> CharacteristicValueSpecification {
        CharacteristicValueSpecification {
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
    pub fn regex(mut self, regex: String) -> Result<CharacteristicValueSpecification,TMFError> {
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
    pub fn value(mut self, value: serde_json::Value) -> Result<CharacteristicValueSpecification,TMFError> {
        self.value_type = Some(serde_value_to_type(&value).to_string());
        match self.regex {
            Some(ref re_str) => {
                let re = Regex::new(&re_str)?;
                let val_str = value.to_string().replace('\"',"");
                if !re.is_match(&val_str) {
                    return Err(TMFError::GenericError(format!("Value {} does not match regex {}",val_str,re_str)));
                }
                self.value = Some(value);
            },
            // If no regex, then just set the value
            None => self.value = Some(value)
        }
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use crate::{HasId, HasName};

    use super::*;

    const SHIPSPEC_JSON: &str = "{
        \"description\" : \"Description\",
        \"id\" : \"SS123\",
        \"isBundle\" : false,
        \"lifecycleStatus\" : \"LifecycleStatus\",
        \"name\" : \"ShipmentSpecificationName\",
        \"version\" : \"4.5\"
    }";

    const SHIPSPECREL_JSON: &str = "{
        \"name\" : \"ShipSpecificationRelationship\"
    }";
    const CHARSPEC_JSON: &str = "{
        \"configurable\" : true,
        \"description\" : \"Description\",
        \"extensible\" : false,
        \"isUnique\" : false,
        \"maxCardinality\" : 2,
        \"minCardinality\" : 1,
        \"name\" : \"CharactaristicSpecificationName\",
        \"regex\" : \"Regex\",
        \"valueType\" :\"String\"
    }";

    const CHARSPECREL_JSON: &str = "{
        \"characteristicSpecificationId\" : \"CS123\",
        \"name\" : \"CharacteristicSpecificationName\",
        \"parentSpecificationHref\" : \"http://example.com/tmf700/specification/CS456\",
        \"parentSpecificationId\" : \"CS456\",
        \"relationshipType\" : \"RelationshipType\"
    }";

    const CHARVALSPEC_JSON: &str = "{
        \"isDefault\" : false,
        \"rangeInterval\" : \"1\",
        \"regex\" : \"Regex\",
        \"unitOfMeasure\" : \"Units\",
        \"value\" : \"2\",
        \"valueFrom\" : 3,
        \"valueTo\" : 4,
        \"valueType\" : \"ValueType\"
    }";
    #[test]
    fn test_shipspec_deserialize() {
        let shipspec: ShipmentSpecificationRefOrValue =
            serde_json::from_str(SHIPSPEC_JSON).unwrap();

        assert_eq!(shipspec.get_description().as_str(), "Description");
        assert_eq!(shipspec.id.is_some(), true);
        assert_eq!(shipspec.get_id().as_str(), "SS123");
        assert_eq!(shipspec.is_bundle, false);
        assert_eq!(shipspec.lifecycle_status.as_str(), "LifecycleStatus");
        assert_eq!(shipspec.name.is_some(), true);
        assert_eq!(shipspec.get_name().as_str(), "ShipmentSpecificationName");
        assert_eq!(shipspec.version.as_str(), "4.5");
    }

    #[test]
    fn test_shipspecrel_deserialize() {
        let shipspecrel: ShipmentSpecificationRelationship =
            serde_json::from_str(SHIPSPECREL_JSON).unwrap();

        assert_eq!(shipspecrel.name.is_some(), true);
        assert_eq!(
            shipspecrel.name.unwrap().as_str(),
            "ShipSpecificationRelationship"
        );
    }

    #[test]
    fn test_charspec_deserialize() {
        let charspec: CharacteristicSpecification = serde_json::from_str(CHARSPEC_JSON).unwrap();

        assert_eq!(charspec.configurable, true);
        assert_eq!(charspec.description.as_str(), "Description");
        assert_eq!(charspec.extensible, false);
        assert_eq!(charspec.is_unique, false);
        assert_eq!(charspec.min_cardinality, 1);
        assert_eq!(charspec.max_cardinality, 2);
        assert_eq!(charspec.name.is_some(), true);
    }

    #[test]
    fn test_charspecrel_deserialize() {
        let charspecrel: CharacteristicSpecificationRelationship =
            serde_json::from_str(CHARSPECREL_JSON).unwrap();

        assert_eq!(
            charspecrel.characteristic_specification_id.as_str(),
            "CS123"
        );
        assert_eq!(charspecrel.name.as_str(), "CharacteristicSpecificationName");
        assert_eq!(charspecrel.parent_specification_id.as_str(), "CS456");
        assert_eq!(charspecrel.relationship_type.as_str(), "RelationshipType");
    }

    #[test]
    fn test_charvalspec_deserialize() {
        let charvalspec: CharacteristicValueSpecification =
            serde_json::from_str(CHARVALSPEC_JSON).unwrap();

        let two_str: String = "2".to_string();
        assert_eq!(charvalspec.is_default, Some(false));
        assert_eq!(charvalspec.range_interval.as_str(), "1");
        assert_eq!(charvalspec.regex.unwrap().as_str(), "Regex");
        assert_eq!(charvalspec.unit_of_measure.as_str(), "Units");
        assert_eq!(charvalspec.value.unwrap().to_string(), two_str);
        assert_eq!(charvalspec.value_from, 3);
        assert_eq!(charvalspec.value_to, 4);
        assert_eq!(charvalspec.value_type.unwrap().as_str(), "ValueType");
    }
}
