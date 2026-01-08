//! Product Specification Module
//!

use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::MOD_PATH;

use crate::common::event::{Event, EventPayload};
use crate::common::tmf_error::TMFError;
use crate::{
    serde_value_to_type, vec_insert, Cardinality, HasDescription, HasId, HasLastUpdate, HasName,
    HasReference, HasValidity, TMFEvent, TimePeriod,
};
use tmflib_derive::{HasDescription, HasId, HasLastUpdate, HasName, HasValidity};

use crate::tmf633::characteristic_specification::CharacteristicSpecification;
use crate::tmf633::service_specification::{ServiceSpecification, ServiceSpecificationRef};

const CLASS_PATH: &str = "productSpecification";
const SPEC_VERS: &str = "1.0";
const CHAR_VALUE_MIN_CARD: Cardinality = 0;
const CHAR_VALUE_MAX_CARD: Cardinality = 1;
/// Verb to tag converted ServiceSpecifications with.
pub const SPEC_CONV_VERB: &str = "Imported";

/// Product Specification Characteristic
#[derive(Clone, Debug, Default, Deserialize, Serialize, HasValidity)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecificationCharacteristic {
    /// Is this characteristic configurable
    pub configurable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extensible: Option<bool>,
    /// Is this characteristic unique
    pub is_unique: bool,
    max_cardinality: Cardinality,
    min_cardinality: Cardinality,
    /// Characteristic Name
    pub name: String,
    /// Regular expression for value
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_type: Option<String>,
    /// Validity period for this characteristic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    /// Set of characteristic relationships
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_spec_char_relationship: Option<Vec<ProductSpecificationCharacteristicRelationship>>,
}

impl ProductSpecificationCharacteristic {
    /// Create a new characteristic
    /// By default, new characteristics are optional (cardinality: min=0 max=1)
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::product_specification::ProductSpecificationCharacteristic;
    /// let ps_char = ProductSpecificationCharacteristic::new(String::from("My Characteristic"));
    /// ```
    pub fn new(name: impl Into<String>) -> ProductSpecificationCharacteristic {
        ProductSpecificationCharacteristic {
            configurable: true,
            max_cardinality: CHAR_VALUE_MAX_CARD,
            min_cardinality: CHAR_VALUE_MIN_CARD,
            name: name.into(),
            valid_for: Some(TimePeriod::default()),
            ..Default::default()
        }
    }

    /// Set configuraable flag
    pub fn configurable(mut self, configurable: bool) -> ProductSpecificationCharacteristic {
        self.configurable = configurable;
        self
    }

    /// Set description of characteristic
    pub fn description(mut self, description: String) -> ProductSpecificationCharacteristic {
        self.description = Some(description.clone());
        self
    }

    /// Set extensible flag
    pub fn extensible(mut self, extensible: bool) -> ProductSpecificationCharacteristic {
        self.extensible = Some(extensible);
        self
    }

    /// Set validity period
    pub fn validity(mut self, validity: Option<TimePeriod>) -> ProductSpecificationCharacteristic {
        self.valid_for = validity;
        self
    }

    /// Set MIN / MAX cardindiality
    /// Will ignore change if min > max.
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::product_specification::ProductSpecificationCharacteristic;
    /// let ps_char = ProductSpecificationCharacteristic::new(String::from("My Characteristic"))
    ///     .cardinality(0,1);
    /// ```
    pub fn cardinality(
        mut self,
        min: Cardinality,
        max: Cardinality,
    ) -> ProductSpecificationCharacteristic {
        // Quick check to make sure min < max
        if min > max {
            // Not sure if we should just ignore this ?
            return self;
        }
        self.min_cardinality = min;
        self.max_cardinality = max;
        self
    }
}

// Conversion from Service CharacteristicSpecification into Product Spec.
impl From<CharacteristicSpecification> for ProductSpecificationCharacteristic {
    fn from(value: CharacteristicSpecification) -> Self {
        ProductSpecificationCharacteristic {
            name: value.name.unwrap_or_default(),
            min_cardinality: value.min_cardinality.unwrap_or_default(),
            max_cardinality: value.max_cardinality.unwrap_or_default(),
            configurable: value.configurable.unwrap_or_default(),
            is_unique: value.is_unique.unwrap_or_default(),
            description: value.description.clone(),
            valid_for: value.valid_for.clone(),
            ..Default::default()
        }
    }
}

/// Bundled Products
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundledProductSpecification {
    id: String,
    href: String,
    lifecycle_status: Option<String>,
    name: String,
}

/// Product Specification
#[derive(
    Clone, Debug, Default, Deserialize, HasId, HasLastUpdate, HasDescription, HasName, Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecification {
    /// Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HREF where object is located
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Brand
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Is a bundle?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bundle: Option<bool>,
    /// Timestamp of last change to this payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
    /// Status of this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_status: Option<String>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Product Number / Code
    #[serde(skip_serializing_if = "Option::is_none")]
    product_number: Option<String>,
    /// Version of this record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Set of characteristics for this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_spec_characteristic: Option<Vec<ProductSpecificationCharacteristic>>,
    /// Bundled specifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundled_product_specification: Option<Vec<BundledProductSpecification>>,
}

impl ProductSpecification {
    /// Create new instance of Product Specification
    pub fn new(name: impl Into<String>) -> ProductSpecification {
        let mut prod_spec = ProductSpecification::create_with_time();
        prod_spec.name = Some(name.into());
        prod_spec.version = Some(SPEC_VERS.to_string());

        prod_spec
    }

    /// Set lifecycle status
    pub fn status(&mut self, status: &str) {
        self.lifecycle_status = Some(status.to_owned());
    }

    /// Add a new Characteristic into the specification
    pub fn with_charateristic(
        mut self,
        characteristic: ProductSpecificationCharacteristic,
    ) -> ProductSpecification {
        vec_insert(&mut self.product_spec_characteristic, characteristic);
        self
    }

    /// Get the class of this object
    pub fn characteristic_by_name(
        &self,
        name: &str,
    ) -> Option<&ProductSpecificationCharacteristic> {
        match self.product_spec_characteristic.as_ref() {
            Some(chars) => chars.iter().find(|c| c.name == name),
            None => None,
        }
    }

    /// Link remote characteristic specification
    pub fn link_characteristic(&mut self, remote: &ProductSpecification, name: impl Into<String>) {
        let name: String = name.into();
        let char_opt = remote.characteristic_by_name(&name);

        if let Some(char_spec) = char_opt {
            let mut new_char = char_spec.clone();
            let char_rel = ProductSpecificationCharacteristicRelationship {
                id: remote.get_id(),
                href: remote.get_href(),
                char_spec_seq: 0,
                name: name.clone(),
                relationship_type: String::from("dependsOn"),
                valid_for: new_char.valid_for.clone(),
            };
            // Insert relationship into placeholder characteristic
            vec_insert(&mut new_char.product_spec_char_relationship, char_rel);
            vec_insert(&mut self.product_spec_characteristic, new_char);
        }
    }
}

/// Product Specification Reference
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductSpecificationRef {
    /// Id
    pub id: String,
    /// HREF where object is located
    pub href: String,
    /// Description
    pub name: Option<String>,
    /// Version of this record
    pub version: Option<String>,
}

impl From<ProductSpecification> for ProductSpecificationRef {
    fn from(ps: ProductSpecification) -> ProductSpecificationRef {
        ProductSpecificationRef {
            id: ps.get_id(),
            href: ps.get_href(),
            name: ps.name.clone(),
            version: ps.version.clone(),
        }
    }
}

impl HasReference for ProductSpecification {
    type RefType = ProductSpecificationRef;
    fn as_ref(&self) -> Option<Self::RefType> {
        Some(ProductSpecificationRef::from(self.clone()))
    }
}

impl From<&ServiceSpecificationRef> for ProductSpecificationRef {
    fn from(value: &ServiceSpecificationRef) -> Self {
        // we cannot simply copy across the href but we can reuse the id
        let mut ps = ProductSpecification {
            id: Some(value.id.clone()),
            name: Some(value.name.clone()),
            ..Default::default()
        };
        ps.generate_href();

        ProductSpecificationRef::from(ps)
    }
}

// Convert a service specification into a peroduct specification
// used as part of the import process.
impl From<&ServiceSpecification> for ProductSpecification {
    fn from(value: &ServiceSpecification) -> Self {
        let mut ps = ProductSpecification::new(value.get_name());
        // get_description() is a method on the ServiceSpecification that always returns a string
        ps.description = Some(format!("{} [{}]", value.get_description(), SPEC_CONV_VERB));
        if value.description.is_some() {
            ps.description = Some(format!("{} [{}]", value.get_description(), SPEC_CONV_VERB));
        }
        ps.is_bundle = value.is_bundle;
        if value.last_update.is_some() {
            ps.set_last_update(value.last_update.as_ref().unwrap());
        }
        if value.spec_characteristics.is_some() {
            // We have characteristics that require conversion
            let mut out: Vec<ProductSpecificationCharacteristic> = Vec::new();
            value
                .spec_characteristics
                .as_ref()
                .unwrap()
                .iter()
                .for_each(|cs| {
                    let psc = ProductSpecificationCharacteristic::from(cs.clone());
                    out.push(psc);
                });
            ps.product_spec_characteristic = Some(out);
        }
        if value.version.is_some() {
            // If source has a version defined take that
            ps.version.clone_from(&value.version);
        }
        ps.lifecycle_status.clone_from(&value.lifecycle_status);
        ps
    }
}

// Events
/// Product Specification Event Container
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProductSpecificationEvent {
    product_specification: ProductSpecification,
}

impl TMFEvent<ProductSpecificationEvent> for ProductSpecification {
    fn event(&self) -> ProductSpecificationEvent {
        ProductSpecificationEvent {
            product_specification: self.clone(),
        }
    }
}

impl EventPayload<ProductSpecificationEvent> for ProductSpecification {
    type Subject = ProductSpecification;
    type EventType = ProductSpecificationEventType;
    fn to_event(
        &self,
        event_type: Self::EventType,
    ) -> Event<ProductSpecificationEvent, Self::EventType> {
        let now = Utc::now();
        let event_time = chrono::DateTime::from_timestamp(now.timestamp(), 0).unwrap();
        let desc = format!("{:?} for {}", event_type, self.get_name());
        Event {
            description: Some(desc),
            domain: Some(ProductSpecification::get_class()),
            event_id: Uuid::new_v4().to_string(),
            href: self.href.clone(),
            id: self.id.clone(),
            title: self.name.clone(),
            event_time: event_time.to_string(),
            event_type,
            event: self.event(),
            ..Event::default()
        }
    }
}

/// Product Specification Event Type
#[derive(Clone, Default, Debug)]
pub enum ProductSpecificationEventType {
    /// Product Specification Created
    #[default]
    ProductSpecificationCreateEvent,
    /// Product Specification Deleted
    ProductSpecificationDeleteEvent,
}

/// Product Specification Characteristic Value
/// # Detalis
/// This object contains values used by a specification characteristic.
/// # Example
/// If the Product Offering is "Internet", then the Specification might be "Bandwidht" and the Value might be "100Mb"
#[derive(Clone, Debug, Default, Deserialize, Serialize, HasValidity)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecificationCharacteristicValue {
    /// Description of Characteristic Value
    pub is_default: bool,
    /// Value Range Interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_interval: Option<String>,
    /// Characteristic Value Regular Expression
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_of_measure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_to: Option<String>,
    /// Type of Characteristic Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    value: serde_json::Value,
}

impl ProductSpecificationCharacteristicValue {
    /// Create a new Product Specification Characteristic Value with a value
    /// # Description
    /// Creates a new Characterisitic Value with the provided value.
    /// Other fields can be set by directly updating the structure.
    /// This bypasses regular experssion checks
    /// # Example
    /// ```
    /// # use tmflib::tmf620::product_specification::ProductSpecificationCharacteristicValue;
    /// let pscv = ProductSpecificationCharacteristicValue::new();
    /// ```
    pub fn new() -> ProductSpecificationCharacteristicValue {
        ProductSpecificationCharacteristicValue {
            is_default: false,
            ..Default::default()
        }
    }

    /// Set the regular expression for this characteristic value
    /// # Example
    /// ```
    /// # use tmflib::tmf620::product_specification::ProductSpecificationCharacteristicValue;
    ///
    /// let pscv = ProductSpecificationCharacteristicValue::new()
    ///     .regex(String::from("[0-9]+(Mb|Gb)"));
    /// ```
    pub fn regex(
        mut self,
        regex: String,
    ) -> Result<ProductSpecificationCharacteristicValue, TMFError> {
        // For now we only wish to test if we can parse the regex string
        let _re = Regex::new(&regex)?;
        self.regex = Some(regex);
        Ok(self)
    }

    /// Set the value for this characteristic value
    /// # Example
    /// ```
    /// # use tmflib::tmf620::product_specification::ProductSpecificationCharacteristicValue;
    /// # use serde_json::json;
    /// let pscv = ProductSpecificationCharacteristicValue::new()
    ///     .regex(String::from("[0-9]+(Mb|Gb)")).unwrap()
    ///     .value("100Mb".into()).unwrap();
    /// ```
    pub fn value(
        mut self,
        value: serde_json::Value,
    ) -> Result<ProductSpecificationCharacteristicValue, TMFError> {
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
                self.value = value;
            }
            // If no regex, then just set the value
            None => self.value = value,
        }
        Ok(self)
    }

    /// Validate a value against the regex (if set) and return an updated
    /// ProductSpecificationCharacteristicValue with the value set.
    /// # Example
    /// ```
    /// # use tmflib::tmf620::product_specification::ProductSpecificationCharacteristicValue;
    /// # use serde_json::json;
    /// let pscv = ProductSpecificationCharacteristicValue::new()
    ///     .regex(String::from("[0-9]+(Mb|Gb)")).unwrap()
    ///    .validate("200Mb".into()).unwrap();
    /// ```
    pub fn validate(
        mut self,
        value: serde_json::Value,
    ) -> Result<ProductSpecificationCharacteristicValue, TMFError> {
        // If we have a regex, then validate the value against it.
        if let Some(re_str) = &self.regex {
            let re = Regex::new(re_str)?;
            let val_str = value.to_string();
            if !re.is_match(&val_str) {
                return Err(TMFError::GenericError(format!(
                    "Value {} does not match regex {}",
                    val_str, re_str
                )));
            }
        }
        self.value = value;
        Ok(self)
    }
}

/// Product Specification Characteristic Value Use
#[derive(Clone, Debug, Deserialize, Serialize, HasValidity)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecificationCharacteristicValueUse {
    /// Description of Characteristic Value Use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    max_cardinality: u16,
    min_cardinality: u16,
    name: String,
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_spec_characteristic_value: Option<Vec<ProductSpecificationCharacteristicValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_specification: Option<ProductSpecificationRef>,
}

impl ProductSpecificationCharacteristicValueUse {
    /// Create a new instance of ProductSpecificationCharacteristicValueUse
    pub fn new(name: impl Into<String>) -> ProductSpecificationCharacteristicValueUse {
        ProductSpecificationCharacteristicValueUse {
            description: None,
            max_cardinality: CHAR_VALUE_MAX_CARD,
            min_cardinality: CHAR_VALUE_MIN_CARD,
            name: name.into(),
            value_type: String::from("String"),
            valid_for: None,
            product_spec_characteristic_value: None,
            product_specification: None,
        }
    }

    /// Add a specificatoin into the ProductSpecificationCharacteristicValueUse
    pub fn with_spec(&mut self, specification: ProductSpecification) {
        self.product_specification = Some(ProductSpecificationRef::from(specification));
    }
}

/// Product Specification Characteristic Relationship
#[derive(Clone, Debug, Deserialize, Serialize, HasValidity)]
pub struct ProductSpecificationCharacteristicRelationship {
    /// Id
    pub id: String,
    /// HREF where object is located
    pub href: String,
    /// Sequence number of the related characteristic
    pub char_spec_seq: u32,
    /// Name of the related characteristic
    pub name: String,
    /// Type of relationship
    pub relationship_type: String,
    /// Validity period for this relationship
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}

#[cfg(test)]
mod test {

    use crate::tmf633::characteristic_specification::CharacteristicSpecification;

    use super::*;
    const SPEC_NAME: &str = "MySpecification";
    const VALUE_NAME: &str = "MyCharValueUse";
    const DESC: &str = "A Description";
    const SERVICE_SPEC: &str = "ServiceSpecification";
    const SPEC_STATUS: &str = "SpecificationStatus";

    #[test]
    fn test_char_value_use_new() {
        let value_use = ProductSpecificationCharacteristicValueUse::new(VALUE_NAME);

        assert_eq!(value_use.name, VALUE_NAME.to_string());
    }

    #[test]
    fn test_char_value_use_new_card() {
        let value_use = ProductSpecificationCharacteristicValueUse::new(VALUE_NAME);

        assert_eq!(value_use.min_cardinality, CHAR_VALUE_MIN_CARD);
        assert_eq!(value_use.max_cardinality, CHAR_VALUE_MAX_CARD);
    }

    #[test]
    fn test_char_value_use_new_value() {
        let value_use = ProductSpecificationCharacteristicValueUse::new(VALUE_NAME);

        assert_eq!(value_use.value_type, String::from("String"));
    }

    #[test]
    fn test_spec_new() {
        let spec = ProductSpecification::new(SPEC_NAME);

        assert_eq!(spec.name, Some(SPEC_NAME.to_string()));
    }
    #[test]
    fn test_spec_new_vers() {
        let spec = ProductSpecification::new(SPEC_NAME);

        assert_eq!(spec.version, Some(SPEC_VERS.to_string()));
    }

    #[test]
    fn test_spec_char_configurable() {
        let spec_char = ProductSpecificationCharacteristic::new(SPEC_NAME).configurable(true);

        assert_eq!(spec_char.configurable, true);
    }

    #[test]
    fn test_spec_char_description() {
        let spec_char =
            ProductSpecificationCharacteristic::new(SPEC_NAME).description(DESC.to_string());

        assert_eq!(spec_char.description.unwrap(), DESC.to_string());
    }

    #[test]
    fn test_spec_extensible() {
        let spec_char = ProductSpecificationCharacteristic::new(SPEC_NAME).extensible(true);

        assert_eq!(spec_char.extensible, Some(true));
    }

    #[test]
    fn test_spec_cardinality() {
        let spec_char = ProductSpecificationCharacteristic::new(SPEC_NAME).cardinality(1, 2);

        assert_eq!(spec_char.min_cardinality, 1);
        assert_eq!(spec_char.max_cardinality, 2);
    }

    #[test]
    fn test_spec_cardinality_invalid() {
        let spec_char = ProductSpecificationCharacteristic::new(SPEC_NAME).cardinality(10, 2);

        // Show in valid setting wont' update anything.
        assert_eq!(spec_char.min_cardinality, CHAR_VALUE_MIN_CARD);
        assert_eq!(spec_char.max_cardinality, CHAR_VALUE_MAX_CARD);
    }

    #[test]
    fn test_spec_from_service_spec() {
        let service_spec = CharacteristicSpecification::new(SERVICE_SPEC);

        let spec = ProductSpecificationCharacteristic::from(service_spec.clone());

        assert_eq!(service_spec.name.unwrap(), spec.name);
        assert_eq!(
            service_spec.min_cardinality.unwrap_or_default(),
            spec.min_cardinality
        );
        assert_eq!(
            service_spec.max_cardinality.unwrap_or_default(),
            spec.max_cardinality
        );
        assert_eq!(service_spec.description, spec.description);
    }

    #[test]
    fn test_spec_status() {
        let mut spec = ProductSpecification::new(SPEC_NAME);

        spec.status(SPEC_STATUS);

        assert_eq!(spec.lifecycle_status.is_some(), true);
        assert_eq!(spec.lifecycle_status.unwrap(), SPEC_STATUS.to_string());
    }

    #[test]
    fn test_spec_with_char() {
        let spec_char1 = ProductSpecificationCharacteristic::new(SPEC_NAME).cardinality(1, 2);
        let spec_char2 = ProductSpecificationCharacteristic::new(SPEC_NAME).cardinality(3, 4);
        let spec = ProductSpecification::new(SPEC_NAME)
            .with_charateristic(spec_char1)
            .with_charateristic(spec_char2);

        assert_eq!(spec.product_spec_characteristic.is_some(), true);
        assert_eq!(spec.product_spec_characteristic.unwrap().len(), 2);
    }

    // #[test]
    // fn test_prodspeccharval_new() {
    //     let pscv = ProductSpecificationCharacteristicValue::new("Value".into());

    //     assert_eq!(pscv.value,"Value".into());
    // }

    #[test]
    fn test_prodspec_asref() {
        let ps = ProductSpecification::new(SPEC_NAME);
        let ps_ref = ps.as_ref().unwrap();

        assert_eq!(ps_ref.id, ps.get_id());
        assert_eq!(ps_ref.href, ps.get_href());
        assert_eq!(ps_ref.name, ps.name);
        assert_eq!(ps_ref.version, ps.version);
    }

    #[test]
    fn test_prodspecvalue_regex() {
        let pscv = ProductSpecificationCharacteristicValue::new()
            .regex(String::from("[0-9]+(Mb|Gb)"))
            .unwrap()
            .value("100Mb".into())
            .unwrap();

        assert_eq!(pscv.regex.is_some(), true);
        assert_eq!(pscv.regex.unwrap(), String::from("[0-9]+(Mb|Gb)"));
        assert_eq!(pscv.value, serde_json::Value::String("100Mb".to_string()));
    }

    #[test]
    fn test_prodspecvalue_regex_invalid() {
        let pscv = ProductSpecificationCharacteristicValue::new()
            .regex(String::from("[0-9]+(Mb|Gb)"))
            .unwrap()
            .value("Invalid".into());
        assert!(pscv.is_err());
    }

    #[test]
    fn test_with_charspec() {
        let spec_char1 = ProductSpecificationCharacteristic::new(SPEC_NAME).cardinality(1, 2);
        let spec_char2 = ProductSpecificationCharacteristic::new(SPEC_NAME).cardinality(3, 4);
        let spec = ProductSpecification::new(SPEC_NAME)
            .with_charateristic(spec_char1)
            .with_charateristic(spec_char2);

        assert_eq!(spec.product_spec_characteristic.is_some(), true);
        assert_eq!(spec.product_spec_characteristic.unwrap().len(), 2);
    }

    #[test]
    fn test_charspec_by_name() {
        let spec_char1 = ProductSpecificationCharacteristic::new("Char1").cardinality(1, 2);
        let spec_char2 = ProductSpecificationCharacteristic::new("Char2").cardinality(3, 4);
        let spec = ProductSpecification::new(SPEC_NAME)
            .with_charateristic(spec_char1)
            .with_charateristic(spec_char2);

        let c1 = spec.characteristic_by_name("Char1");
        assert!(c1.is_some());
        assert_eq!(c1.unwrap().name, "Char1".to_string());

        let c2 = spec.characteristic_by_name("Char2");
        assert!(c2.is_some());
        assert_eq!(c2.unwrap().name, "Char2".to_string());

        let c3 = spec.characteristic_by_name("Char3");
        assert!(c3.is_none());
    }

    #[test]
    fn test_link_characteristic() {
        let spec_char1 = ProductSpecificationCharacteristic::new("Char1").cardinality(1, 2);
        let mut spec1 = ProductSpecification::new("Spec1").with_charateristic(spec_char1);
        let spec_char2 = ProductSpecificationCharacteristic::new("Char2").cardinality(3, 4);
        let spec2 = ProductSpecification::new("Spec2").with_charateristic(spec_char2);

        spec1.link_characteristic(&spec2, "Char2");

        assert!(spec1.product_spec_characteristic.is_some());
        let chars = spec1.product_spec_characteristic.unwrap();
        assert_eq!(chars.len(), 2);
        let linked_char = chars.iter().find(|c| c.name == "Char2".to_string());
        assert!(linked_char.is_some());
        let rels = &linked_char.unwrap().product_spec_char_relationship;
        assert!(rels.is_some());
        let rels = rels.as_ref().unwrap();
        assert_eq!(rels.len(), 1);
        let rel = &rels[0];
        assert_eq!(rel.name, "Char2".to_string());
        assert_eq!(rel.relationship_type, "dependsOn".to_string());
        assert_eq!(rel.id, spec2.get_id());
        assert_eq!(rel.href, spec2.get_href());
    }
}
