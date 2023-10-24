//! Product Specification Module
//!

use serde::{Deserialize, Serialize};

use super::MOD_PATH;

use crate::{HasId,CreateTMF,LIB_PATH};

const SPEC_PATH: &str = "productSpecification";
const SPEC_VERS: &str = "1.0";
const CHAR_VALUE_MIN_CARD : u16 = 0;
const CHAR_VALUE_MAX_CARD : u16 = 1;

/// Product Specification Characteristic
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecificationCharacteristic {
    configurable: bool,
    description: Option<String>,
    extensible: bool,
    is_unique: bool,
    max_cardinality: u16,
    min_cardinality: u16,
    name: String,
    regex: Option<String>,
    value_type: Option<String>,
    valid_for: Option<String>,
}

impl ProductSpecificationCharacteristic {
    /// Create a new characteristic
    /// By default, new characteristics are optional (cardinality: min=0 max=1)
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::product_specification::ProductSpecificationCharacteristic;
    /// let ps_char = ProductSpecificationCharacteristic::new(String::from("My Characteristic"));
    /// ```
    pub fn new(name: String) -> ProductSpecificationCharacteristic {
        ProductSpecificationCharacteristic {
            configurable: true,
            max_cardinality: CHAR_VALUE_MAX_CARD,
            min_cardinality: CHAR_VALUE_MIN_CARD,
            name,
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
        self.extensible = extensible;
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

/// Product Specification
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductSpecification {
    /// Id
    pub id: Option<String>,
    /// HREF where object is located
    pub href: Option<String>,
    /// Brand
    pub brand: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Is a bundle?
    pub is_bundle: bool,
    /// Name
    pub name: String,
    /// Version of this record
    pub version: Option<String>,
    /// Set of characteristics for this specification
    pub product_spec_characteristic: Vec<ProductSpecificationCharacteristic>,
}

impl ProductSpecification {
    /// Create new instance of Product Specification
    pub fn new(name: String) -> ProductSpecification {       
        let mut prod_spec = ProductSpecification::create();
        prod_spec.name = name;
        prod_spec.version = Some(SPEC_VERS.to_string());
        prod_spec
    }

    /// Add a new Characteristic into the specification
    pub fn with_charateristic(
        mut self,
        characteristic: ProductSpecificationCharacteristic,
    ) -> ProductSpecification {
        self.product_spec_characteristic.push(characteristic);
        self
    }
}

impl CreateTMF<ProductSpecification> for ProductSpecification {}

impl HasId for ProductSpecification {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}", LIB_PATH, MOD_PATH, SPEC_PATH, self.get_id());
        self.href = Some(href);    
    }
    fn generate_id(&mut self) {
        let id = ProductSpecification::get_uuid();
        self.id = Some(id);
        self.generate_href();    
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()    
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()
    }
    fn get_class() -> String {
        SPEC_PATH.to_owned()
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
    pub name: String,
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
    value_for: Option<String>,
    value: String,
}

impl ProductSpecificationCharacteristicValue {
    /// Create a new Product Specification Characteristic Value with a value
    /// # Example
    /// ```
    /// # use tmflib::tmf620::product_specification::ProductSpecificationCharacteristicValue;
    /// let pscv = ProductSpecificationCharacteristicValue::new(String::from("100Mb"));
    /// ```
    pub fn new(value : String) -> ProductSpecificationCharacteristicValue {
        ProductSpecificationCharacteristicValue { value, ..Default::default() }
    }
}

/// Product Specification Characteristic Value Use
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductSpecificationCharacteristicValueUse {
    description: Option<String>,
    max_cardinality: u16,
    min_cardinality: u16,
    name: String,
    value_type: String,
    valid_for: Option<String>,
    product_spec_characteristic_value : Option<ProductSpecificationCharacteristicValue>,
    product_specification : Option<ProductSpecificationRef>,
}

impl ProductSpecificationCharacteristicValueUse {
    /// Create a new instance of ProductSpecificationCharacteristicValueUse
    pub fn new(name : String) -> ProductSpecificationCharacteristicValueUse {
        ProductSpecificationCharacteristicValueUse { 
            description: None, 
            max_cardinality: 1, 
            min_cardinality: 0, 
            name, 
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

        assert_eq!(spec.name, SPEC_NAME.to_string());
    }
    #[test]
    fn test_spec_new_vers() {
        let spec = ProductSpecification::new(SPEC_NAME.to_string());

        assert_eq!(spec.version, Some(SPEC_VERS.to_string()));
    }
}
