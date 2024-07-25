//! Product Specification Module
//!

use serde::{Deserialize, Serialize};

use super::MOD_PATH;

use crate::{HasId, HasName, LIB_PATH,HasValidity, TimePeriod, HasLastUpdate,Cardinality};
use tmflib_derive::{HasId,HasLastUpdate,HasName,HasValidity};

use crate::tmf633::service_specification::{ServiceSpecification,ServiceSpecificationRef};
use crate::tmf633::characteristic_specification::CharacteristicSpecification;

const CLASS_PATH: &str = "productSpecification";
const SPEC_VERS: &str = "1.0";
const CHAR_VALUE_MIN_CARD : Cardinality = 0;
const CHAR_VALUE_MAX_CARD : Cardinality = 1;

/// Product Specification Characteristic
#[derive(Clone, Debug, Default, Deserialize, Serialize, HasValidity)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecificationCharacteristic {
    configurable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extensible: Option<bool>,
    is_unique: bool,
    max_cardinality: Cardinality,
    min_cardinality: Cardinality,
    /// Characteristic Name
    pub name: String,
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

    /// Set MIN / MAX cardindiality
    /// Will ignore change if min > max.
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::product_specification::ProductSpecificationCharacteristic;
    /// let ps_char = ProductSpecificationCharacteristic::new(String::from("My Characteristic"))
    ///     .cardinality(0,1);
    /// ```
    pub fn cardinality(mut self, min: Cardinality, max: Cardinality) -> ProductSpecificationCharacteristic {
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
            name : value.name.unwrap_or_default(),
            min_cardinality : value.min_cardinality.unwrap_or_default(),
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
#[derive(Clone, Debug, Default, Deserialize, HasId, HasLastUpdate, HasName, Serialize)]
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
    pub fn status(&mut self, status : &str) {
        self.lifecycle_status = Some(status.to_owned());
    }

    /// Add a new Characteristic into the specification
    pub fn with_charateristic(
        mut self,
        characteristic: ProductSpecificationCharacteristic,
    ) -> ProductSpecification {
        match self.product_spec_characteristic.as_mut() {
            Some(v) => {
                v.push(characteristic)
            },
            None => self.product_spec_characteristic = Some(vec![characteristic]),
        }
        self
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

impl From<&ServiceSpecificationRef> for ProductSpecificationRef {
    fn from(value: &ServiceSpecificationRef) -> Self {
        // we cannot simply copy across the href but we can reuse the id        
        let mut ps = ProductSpecification {
            id : Some(value.id.clone()),
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
        let mut ps = ProductSpecification::new(format!("{} [Converted from Service Spec]",value.get_name()));
        if value.description.is_some() {
            ps.description = Some(value.description.as_ref().unwrap().clone());
        }
        ps.is_bundle = value.is_bundle;
        if value.last_update.is_some() {
            ps.set_last_update(value.last_update.as_ref().unwrap());
        }
        if value.spec_characteristics.is_some() {
            // We have characteristics that require conversion
            let mut out : Vec<ProductSpecificationCharacteristic> = Vec::new();
            value.spec_characteristics.as_ref().unwrap().iter().for_each(|cs| {
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
#[derive(Clone, Debug ,Default , Deserialize, Serialize, HasValidity)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecificationCharacteristicValue {
    is_default: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_of_measure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(Clone, Debug, Deserialize, Serialize, HasValidity)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecificationCharacteristicValueUse {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    max_cardinality: u16,
    min_cardinality: u16,
    name: String,
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_spec_characteristic_value : Option<Vec<ProductSpecificationCharacteristicValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_specification : Option<ProductSpecificationRef>,
}

impl ProductSpecificationCharacteristicValueUse {
    /// Create a new instance of ProductSpecificationCharacteristicValueUse
    pub fn new(name : impl Into<String>) -> ProductSpecificationCharacteristicValueUse {
        ProductSpecificationCharacteristicValueUse { 
            description: None, 
            max_cardinality: CHAR_VALUE_MAX_CARD, 
            min_cardinality: CHAR_VALUE_MIN_CARD, 
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

    use crate::tmf633::characteristic_specification::CharacteristicSpecification;

    use super::*;
    const SPEC_NAME: &str = "MySpecification";
    const VALUE_NAME: &str = "MyCharValueUse";
    const DESC : &str = "A Description";
    const SERVICE_SPEC : &str = "ServiceSpecification";
    const SPEC_STATUS: &str = "SpecificationStatus";
    const ENUM_STR : &str = "ValueEnumString";

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
        let spec_char = ProductSpecificationCharacteristic::new(SPEC_NAME)
        .configurable(true);

        assert_eq!(spec_char.configurable,true);
    }

    #[test]
    fn test_spec_char_description() {
        let spec_char = ProductSpecificationCharacteristic::new(SPEC_NAME)
        .description(DESC.to_string()); 

        assert_eq!(spec_char.description.unwrap(),DESC.to_string());   
    }

    #[test]
    fn test_spec_extensible() {
        let spec_char = ProductSpecificationCharacteristic::new(SPEC_NAME)
            .extensible(true);

        assert_eq!(spec_char.extensible,Some(true));    
    }

    #[test]
    fn test_spec_cardinality() {
        let spec_char = ProductSpecificationCharacteristic::new(SPEC_NAME)
            .cardinality(1, 2);

        assert_eq!(spec_char.min_cardinality,1);
        assert_eq!(spec_char.max_cardinality,2);
    }

    #[test]
    fn test_spec_cardinality_invalid() {
        let spec_char = ProductSpecificationCharacteristic::new(SPEC_NAME)
        .cardinality(10, 2); 

        // Show in valid setting wont' update anything.
        assert_eq!(spec_char.min_cardinality,CHAR_VALUE_MIN_CARD);
        assert_eq!(spec_char.max_cardinality,CHAR_VALUE_MAX_CARD);   
    }

    #[test]
    fn test_spec_from_service_spec() {
        let service_spec = CharacteristicSpecification::new(SERVICE_SPEC);

        let spec = ProductSpecificationCharacteristic::from(service_spec.clone());

        assert_eq!(service_spec.name.unwrap(),spec.name);
        assert_eq!(service_spec.min_cardinality.unwrap_or_default(),spec.min_cardinality);
        assert_eq!(service_spec.max_cardinality.unwrap_or_default(),spec.max_cardinality);
        assert_eq!(service_spec.description,spec.description);
    }

    #[test]
    fn test_spec_status() {
        let mut spec = ProductSpecification::new(SPEC_NAME);

        spec.status(SPEC_STATUS);

        assert_eq!(spec.lifecycle_status.is_some(),true);
        assert_eq!(spec.lifecycle_status.unwrap(),SPEC_STATUS.to_string());
    }

    #[test]
    fn test_spec_with_char() {
        let spec_char1 = ProductSpecificationCharacteristic::new(SPEC_NAME)
            .cardinality(1, 2);
        let spec_char2 = ProductSpecificationCharacteristic::new(SPEC_NAME)
            .cardinality(3, 4);
        let spec = ProductSpecification::new(SPEC_NAME)
            .with_charateristic(spec_char1)
            .with_charateristic(spec_char2);

        assert_eq!(spec.product_spec_characteristic.is_some(),true);
        assert_eq!(spec.product_spec_characteristic.unwrap().len(),2);        
    }

    fn enum_to_type(value : ValueEnum) -> String {
        let enum_type = match value {
            ValueEnum::Str(_) => "Str",
            ValueEnum::Int(_) => "Int",
            ValueEnum::BadValue => "Bad",
        };
        enum_type.to_string()
    }

    #[test]
    fn test_valueenum_from_string() {
        let value = ValueEnum::from(ENUM_STR.to_string());

        assert_eq!(enum_to_type(value),"Str");
    }

    #[test]
    fn test_valueenum_from_str() {
        let value = ValueEnum::from(ENUM_STR);

        assert_eq!(enum_to_type(value),"Str");
    }

    #[test]
    fn test_valueenum_from_u16() {
        let value = ValueEnum::from(16 as u16);

        assert_eq!(enum_to_type(value),"Int");
    }

    #[test]
    fn test_valueenum_default() {
        let value = ValueEnum::default();

        assert_eq!(enum_to_type(value),"Bad");
    }

    #[test]
    fn test_prodspeccharval_new() {
        let pscv = ProductSpecificationCharacteristicValue::new(ValueEnum::from("Value"));

        assert_eq!(enum_to_type(pscv.value),"Str");
    }
}