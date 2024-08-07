//! Shipment Specification Module

use serde::{Deserialize,Serialize};
use crate::common::attachment::AttachmentRefOrValue;
use tmflib_derive::{
    HasId,
    HasAttachment,
    HasName,
    HasLastUpdate,
    HasValidity
};

use crate::{
    LIB_PATH,
    Uri,
    DateTime,
    TimePeriod,
    HasId,
    HasAttachment,
    HasName,
    HasLastUpdate,
    HasValidity,
};

use super::MOD_PATH;
const CLASS_PATH: &str = "shippingSpecification";

/// Shipment Specification
#[derive(Clone,Default,Debug,Deserialize,HasId,HasAttachment,HasName,HasLastUpdate,HasValidity,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentSpecificationRefOrValue {
    /// Description
    pub description: String,
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
    shipment_spec_relationship : Option<Vec<ShipmentSpecificationRelationship>>,
    /// Shipment Specification Characteristic
    shipment_specification_characteristic: Option<Vec<CharacteristicSpecification>>,
    /// Attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentRefOrValue>>,
}

/// Shipment Specification Relationship
#[derive(Clone,Default,Debug,HasId,HasName,HasValidity,Deserialize,Serialize)]
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
    role:  Option<String>,
    /// Validity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}

/// Shipping Characteristic Specification
#[derive(Clone,Default,Debug,HasValidity,Deserialize,Serialize)]
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
    name : Option<String>,
    regex : String,
    valid_for: Option<TimePeriod>,
    value_type: String,
    // Referenced Struct
    characteristic_value_specification: Option<Vec<CharacteristicValueSpecification>>,
    char_spec_relationship : Option<Vec<CharacteristicSpecificationRelationship>>,
}

/// Shipping Order Characteristic Speficiation
#[derive(Clone,Default,Debug,HasValidity,Deserialize,Serialize)]
pub struct CharacteristicSpecificationRelationship {
    characteristic_specification_id : String,
    name : String,
    parent_specification_href: Uri,
    parent_specification_id: String,
    relationship_type: String,
    valid_for: Option<TimePeriod>,
}


/// Shipping Characteristic Value Specification
#[derive(Clone,Default,Debug,HasValidity,Deserialize,Serialize)]
pub struct CharacteristicValueSpecification {
    is_default: bool,
    range_interval: String,
    regex: String,
    unit_of_measure: String,
    valid_for: Option<TimePeriod>,
    value: String,
    value_from: u16,
    value_to: u16,
    value_type: String,
}

#[cfg(test)]
mod test {
    use crate::{HasId, HasName};

    use super::{CharacteristicSpecification, ShipmentSpecificationRefOrValue, ShipmentSpecificationRelationship};


    const SHIPSPEC_JSON : &str = "{
        \"description\" : \"Description\",
        \"id\" : \"SS123\",
        \"isBundle\" : false,
        \"lifecycleStatus\" : \"LifecycleStatus\",
        \"name\" : \"ShipmentSpecificationName\",
        \"version\" : \"4.5\"
    }";

    const SHIPSPECREL_JSON : &str = "{
        \"name\" : \"ShipSpecificationRelationship\"
    }";
    const CHARSPEC_JSON : &str = "{
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
   #[test]
   fn test_shipspec_deserialize() {
        let shipspec : ShipmentSpecificationRefOrValue = serde_json::from_str(SHIPSPEC_JSON).unwrap();

        assert_eq!(shipspec.description.as_str(),"Description");
        assert_eq!(shipspec.id.is_some(),true);
        assert_eq!(shipspec.get_id().as_str(),"SS123");
        assert_eq!(shipspec.is_bundle,false);
        assert_eq!(shipspec.lifecycle_status.as_str(),"LifecycleStatus");
        assert_eq!(shipspec.name.is_some(),true);
        assert_eq!(shipspec.get_name().as_str(),"ShipmentSpecificationName");
        assert_eq!(shipspec.version.as_str(),"4.5");
   }

    #[test]
    fn test_shipspecrel_deserialize() {
        let shipspecrel : ShipmentSpecificationRelationship = serde_json::from_str(SHIPSPECREL_JSON).unwrap();

        assert_eq!(shipspecrel.name.is_some(),true);
        assert_eq!(shipspecrel.name.unwrap().as_str(),"ShipSpecificationRelationship");
    }

    #[test]
    fn test_charspec_deserialize() {
        let charspec : CharacteristicSpecification = serde_json::from_str(CHARSPEC_JSON).unwrap();
    }

   #[test]
   fn test_charspecrel_deserialize() {}

   #[test]
   fn test_charvalspec_deserialize() {}

}