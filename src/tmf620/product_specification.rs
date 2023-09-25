//! Product Specification Module
//! 

use serde::{Deserialize,Serialize};
use uuid::Uuid;

use super::LIB_PATH;
use super::MOD_PATH;

const SPEC_PATH : &str = "productSpecification";
const SPEC_VERS : &str = "1.0";

/// Product Specification Characteristic
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct ProductSpecificationCharacteristic {
    configurable    : bool,
    description     : Option<String>,
    extensible      : bool,
    is_unique       : bool,
    max_cardinality : u16,
    min_cardinality : u16,
    name            : String,
    regex           : Option<String>,
    value_type      : Option<String>,
    valid_for       : Option<String>,
}

impl ProductSpecificationCharacteristic {
    /// Create a new characteristic
    /// By default, new characteristics are optional (cardinality: min=0 max=1)
    /// # Examples
    /// ```
    /// # use tmflib::tmf620::product_specification::ProductSpecificationCharacteristic;
    /// let ps_char = ProductSpecificationCharacteristic::new(String::from("My Characteristic"));
    /// ```
    pub fn new(name : String) -> ProductSpecificationCharacteristic {
        ProductSpecificationCharacteristic { 
            configurable    : true, 
            description     : None, 
            extensible      : false, 
            is_unique       : false, 
            max_cardinality : 1, 
            min_cardinality : 0, 
            name, 
            regex           : None, 
            value_type      : None, 
            valid_for       : None 
        }
    }

    /// Set configuraable flag
    pub fn configurable(mut self, configurable : bool) -> ProductSpecificationCharacteristic {
        self.configurable = configurable;
        self
    }

    /// Set description of characteristic
    pub fn description(mut self, description : String) -> ProductSpecificationCharacteristic {
        self.description = Some(description.clone());
        self    
    }

    /// Set extensible flag
    pub fn extensible( mut self, extensible : bool) -> ProductSpecificationCharacteristic {
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
    pub fn cardinality ( mut self, min : u16, max: u16) -> ProductSpecificationCharacteristic {
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
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct ProductSpecification {
    /// Id
    pub id : String,
    /// HREF where object is located
    pub href : String,
    /// Brand
    pub brand : Option<String>,
    /// Description
    pub description : Option<String>,
    /// Is a bundle?
    pub is_bundle : bool,
    /// Name
    pub name : String,
    /// Version of this record
    pub version : Option<String>,
    /// Set of characteristics for this specification
    pub product_spec_characteristic : Vec<ProductSpecificationCharacteristic>,  
}

impl ProductSpecification {
    /// Create new instance of Product Specification
    pub fn new(name: String) -> ProductSpecification {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,SPEC_PATH,id);
        ProductSpecification { 
            id, 
            href,
            brand : None,
            description : None,
            is_bundle : false, 
            name,
            version: Some(SPEC_VERS.to_string()),
            product_spec_characteristic : vec![],
        }
    }
}

/// Product Specification Reference
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct ProductSpecificationRef {
    /// Id
    pub id : String,
    /// HREF where object is located
    pub href : String,
    /// Description
    pub name : String,
    /// Version of this record
    pub version : Option<String>,
}

impl From<ProductSpecification> for ProductSpecificationRef {
    fn from(ps : ProductSpecification) -> ProductSpecificationRef {
        ProductSpecificationRef { id: ps.id, href: ps.href, name: ps.name, version: ps.version }
    }
}

/// Product Specification Characteristic Value Use
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct ProductSpecificationCharacteristicValueUse {}


#[cfg(test)]
mod test {

    use super::ProductSpecification;
    use super::SPEC_VERS;
    const SPEC_NAME: &str = "MySpecification";

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