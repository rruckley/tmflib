//! Product Specification Module
//!

use serde::{Deserialize, Serialize};

use super::MOD_PATH;

use crate::{HasId, HasName, CreateTMF, LIB_PATH, TimePeriod, CreateTMFWithTime, HasLastUpdate};
use tmflib_derive::{HasId,HasLastUpdate};

const CLASS_PATH: &str = "productSpecification";
const SPEC_VERS: &str = "1.0";
const CHAR_VALUE_MIN_CARD : u16 = 0;
const CHAR_VALUE_MAX_CARD : u16 = 1;

/// Product Specification Characteristic
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecificationCharacteristic {
    configurable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extensible: Option<bool>,
    is_unique: bool,
    max_cardinality: u16,
    min_cardinality: u16,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
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
            name : name.into(),
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

    /// Set MIN / MAX cardindiality
    /// Will ignore change if min > max.
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::product_specification::ProductSpecificationCharacteristic;
    /// let ps_char = ProductSpecificationCharacteristic::new(String::from("My Characteristic"))
    ///     .cardinality(0,1);
    /// ```
    pub fn cardinality(mut self, min: u16, max: u16) -> ProductSpecificationCharacteristic {
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
#[derive(Clone, Debug, Default, Deserialize, HasId, HasLastUpdate, Serialize)]
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

        prod_spec.product_spec_characteristic= Some(vec![]);
        prod_spec
    }

    /// Set lifecycle status
    pub fn status(&mut self, status : &str) {
        self.lifecycle_status = Some(status.to_owned());
    }

    /// Add a new Characteristic into the specification
    pub fn with_charateristic(
        mut self,
        characteristic: ProductSpecificationCharacteristic,
    ) -> ProductSpecification {
        self.product_spec_characteristic.as_mut().unwrap().push(characteristic);
        self
    }
}

impl HasName for ProductSpecification {
    fn get_name(&self) -> String {
        self.name.as_ref().unwrap().clone()
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
            id: ps.id.unwrap(),
            href: ps.href.unwrap(),
            name: ps.name,
            version: ps.version,
        }
    }
}

/// Enum to cover value with many types
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ValueEnum {
    /// Unknown type
    #[default]
    BadValue,
    /// String Value
    Str(String),
    /// Integer Value
    Int(u16),
}

impl From<String> for ValueEnum {
    fn from(value: String) -> Self {
        ValueEnum::Str(value)
    }
}

impl From<u16> for ValueEnum {
    fn from(value: u16) -> Self {
        ValueEnum::Int(value)
    }
}

impl From<&str> for ValueEnum {
    fn from(value: &str) -> Self {
        ValueEnum::Str(value.to_owned())
    }
}

/// Product Specification Characteristic Value
/// # Detalis
/// This object contains values used by a specification characteristic.
/// # Example
/// If the Product Offering is "Internet", then the Specification might be "Bandwidht" and the Value might be "100Mb"
#[derive(Clone, Debug ,Default , Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecificationCharacteristicValue {
    is_default: bool,
    range_interval: Option<String>,
    regex: Option<String>,
    unit_of_measure: Option<String>,
    value_from: Option<String>,
    value_to: Option<String>,
    value_type: Option<String>,
    valid_for: Option<TimePeriod>,
    value: ValueEnum,
}

impl ProductSpecificationCharacteristicValue {
    /// Create a new Product Specification Characteristic Value with a value
    /// # Example
    /// ```
    /// # use tmflib::tmf620::product_specification::ProductSpecificationCharacteristicValue;
    /// let pscv = ProductSpecificationCharacteristicValue::new("100Mb".into());
    /// ```
    pub fn new(value : ValueEnum) -> ProductSpecificationCharacteristicValue {
        ProductSpecificationCharacteristicValue { value, ..Default::default() }
    }
}

/// Product Specification Characteristic Value Use
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecificationCharacteristicValueUse {
    description: Option<String>,
    max_cardinality: u16,
    min_cardinality: u16,
    name: String,
    value_type: String,
    valid_for: Option<TimePeriod>,
    product_spec_characteristic_value : Option<Vec<ProductSpecificationCharacteristicValue>>,
    product_specification : Option<ProductSpecificationRef>,
}

impl ProductSpecificationCharacteristicValueUse {
    /// Create a new instance of ProductSpecificationCharacteristicValueUse
    pub fn new(name : impl Into<String>) -> ProductSpecificationCharacteristicValueUse {
        ProductSpecificationCharacteristicValueUse { 
            description: None, 
            max_cardinality: 1, 
            min_cardinality: 0, 
            name : name.into(), 
            value_type: String::from("String"), 
            valid_for: None,
            product_spec_characteristic_value : None,
            product_specification : None, 
        }
    }

    /// Add a specificatoin into the ProductSpecificationCharacteristicValueUse
    pub fn with_spec(&mut self, specification : ProductSpecification) {
        self.product_specification = Some(ProductSpecificationRef::from(specification));
    }
}

#[cfg(test)]
mod test {

    use super::ProductSpecification;
    use super::ProductSpecificationCharacteristicValueUse;
    use super::SPEC_VERS;
    use super::CHAR_VALUE_MIN_CARD;
    use super::CHAR_VALUE_MAX_CARD;
    const SPEC_NAME: &str = "MySpecification";
    const VALUE_NAME: &str = "MyCharValueUse";

    #[test]
    fn test_char_value_use_new() {
        let value_use = ProductSpecificationCharacteristicValueUse::new(VALUE_NAME.to_string());

        assert_eq!(value_use.name, VALUE_NAME.to_string());
    }

    #[test]
    fn test_char_value_use_new_card() {
        let value_use = ProductSpecificationCharacteristicValueUse::new(VALUE_NAME.to_string());

        assert_eq!(value_use.min_cardinality, CHAR_VALUE_MIN_CARD);
        assert_eq!(value_use.max_cardinality, CHAR_VALUE_MAX_CARD);
    }

    #[test]
    fn test_char_value_use_new_value() {
        let value_use = ProductSpecificationCharacteristicValueUse::new(VALUE_NAME.to_string());

        assert_eq!(value_use.value_type, String::from("String"));
    }

    #[test]
    fn test_spec_new() {
        let spec = ProductSpecification::new(SPEC_NAME.to_string());

        assert_eq!(spec.name, Some(SPEC_NAME.to_string()));
    }
    #[test]
    fn test_spec_new_vers() {
        let spec = ProductSpecification::new(SPEC_NAME.to_string());

        assert_eq!(spec.version, Some(SPEC_VERS.to_string()));
    }
}
