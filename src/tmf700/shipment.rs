//! Shipment Module

use crate::common::attachment::AttachmentRefOrValue;
use crate::common::external_identifier::ExternalIdentifier;
use crate::{DateTime, HasAttachment, HasId, HasName, Quantity, Uri, LIB_PATH};
use serde::{Deserialize, Serialize};
use tmflib_derive::{HasAttachment, HasId, HasName};

use super::shipment_specification::ShipmentSpecificationRefOrValue;

use super::MOD_PATH;
const CLASS_PATH: &str = "shipment";

/// Shipment Tracking
#[derive(Clone, Default, Debug, Deserialize, HasId, HasName, Serialize)]
pub struct ShipmentTrackingRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

/// Shipment Item Action Type
#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
pub enum ShipmentItemActionType {
    /// Add new item
    Add,
    /// Modify existing item
    Modify,
    /// Delete existing item
    Delete,
    /// No change to existing item
    #[default]
    NoChange,
}
/**
 * Move these two into TMF687 when it gets created
 */
/// Product Stock Reference
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ProductStockRef {
    id: String,
    href: Uri,
    name: String,
}

/// Reseved Product Stock Reference
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ReservedProductStockRef {
    id: String,
    href: Uri,
    name: String,
}

/// Shipment Item - Individual piece of eqiupment to be delivered
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct ShipmentItem {
    /// Action
    pub action: ShipmentItemActionType,
    /// Id
    id: String,
    /// Number of items
    pub quantity: String,
    /// SKU
    pub sku: String,
    /// Weight of item
    pub weight: Quantity,
    // Referenced structs
    /// Product Stock Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_stock_ref: Option<ProductStockRef>,
    /// Product Reservation Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_reservation_ref: Option<ReservedProductStockRef>,
}

/// Shipment
#[derive(Clone, Default, Debug, Deserialize, HasId, HasAttachment, HasName, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentRefOrValue {
    /// Collection Date
    pub collection_date: String,
    /// Completion Date
    pub completion_date: DateTime,
    /// Delivery Date
    pub delivery_date: DateTime,
    /// Description
    pub description: String,
    /// Expeected Delivery Date
    pub expected_delivery_date: String,
    /// HTTP Reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Requested Delivery Date
    pub requested_delivery_date: DateTime,
    /// Status
    pub state: String,
    weight: Quantity,
    // Referenced structs
    /// Attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentRefOrValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipment_specification: Option<ShipmentSpecificationRefOrValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipment_tracking: Option<ShipmentTrackingRef>,
    /// Set of external identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_identifier: Option<Vec<ExternalIdentifier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Shipment Item - Individual equipment items
    pub shipment_item: Option<Vec<ShipmentItem>>,
}

#[cfg(test)]
mod test {
    use super::{
        ProductStockRef, ReservedProductStockRef, ShipmentItem, ShipmentItemActionType,
        ShipmentRefOrValue, ShipmentTrackingRef,
    };
    use crate::{HasId, HasName};

    const SHIPTRACKREF_JSON: &str = "{
        \"id\" : \"STR123\",
        \"href\" : \"http://example.com/tmf700/shipment/STR123\",
        \"name\" : \"ShipmentTrackingRef\"
    }";
    const SHIPITEMACTION_JSON: &str = "\"NoChange\"";
    const PRODSTOCKREF_JSON: &str = "{
        \"id\" : \"PSR123\",
        \"href\" : \"http://example.com/tmf700/shipment/PSR123\",
        \"name\" : \"ProductStockRef\"
    }";
    const RESPRODSTOCKREF_JSON: &str = "{
        \"id\" : \"RPSR123\",
        \"href\" : \"http://example.com/tmf700/shipment/RPSR123\",
        \"name\" : \"ReservedProductStockRef\"
    }";
    const SHIPITEM_JSON: &str = "{
        \"action\" : \"Add\",
        \"id\" : \"SI123\",
        \"quantity\" : \"4\",
        \"sku\" : \"SKU567A\",
        \"weight\" : {
            \"units\" : \"kg\",
            \"amount\" : 8.9
        }
    }";

    const SHIPMENT_JSON: &str = "{
        \"collectionDate\" : \"2024-01-01\",
        \"completionDate\" : \"2024-02-02\",
        \"deliveryDate\" : \"2024-03-03\",
        \"description\" : \"ShipmentDescription\",
        \"expectedDeliveryDate\" : \"2024-04-04\",
        \"name\" : \"Hardware Shipment\",
        \"requestedDeliveryDate\" : \"2024-05-05\",
        \"state\" : \"New\",
        \"weight\" : {
            \"units\" : \"kg\",
            \"amount\" : 6.7
        }
    }";
    #[test]
    fn test_shiptrackref_deseralize() {
        let shiptrackref: ShipmentTrackingRef = serde_json::from_str(SHIPTRACKREF_JSON).unwrap();

        assert_eq!(shiptrackref.id.is_some(), true);
        assert_eq!(shiptrackref.get_id().as_str(), "STR123");
        assert_eq!(shiptrackref.name.is_some(), true);
        assert_eq!(shiptrackref.get_name(), "ShipmentTrackingRef");
    }

    #[test]
    fn test_shipitemaction_deseralize() {
        let shipitemaction: ShipmentItemActionType =
            serde_json::from_str(SHIPITEMACTION_JSON).unwrap();

        assert_eq!(shipitemaction, ShipmentItemActionType::NoChange);
    }

    #[test]
    fn test_prodstockref_deserialize() {
        let prodstockref: ProductStockRef = serde_json::from_str(PRODSTOCKREF_JSON).unwrap();

        assert_eq!(prodstockref.id.as_str(), "PSR123");
        assert_eq!(prodstockref.name.as_str(), "ProductStockRef");
    }

    #[test]
    fn test_resprodstockref_deseralize() {
        let resprodstockref: ReservedProductStockRef =
            serde_json::from_str(RESPRODSTOCKREF_JSON).unwrap();

        assert_eq!(resprodstockref.id.as_str(), "RPSR123");
    }

    #[test]
    fn test_shipitem_deserialize() {
        let shipitem: ShipmentItem = serde_json::from_str(SHIPITEM_JSON).unwrap();

        assert_eq!(shipitem.id.as_str(), "SI123");
        assert_eq!(shipitem.action, ShipmentItemActionType::Add);
        assert_eq!(shipitem.quantity.as_str(), "4");
    }

    #[test]
    fn test_shipment_deserialize() {
        let shipment: ShipmentRefOrValue = serde_json::from_str(SHIPMENT_JSON).unwrap();

        assert_eq!(shipment.description.as_str(), "ShipmentDescription");
    }
}
