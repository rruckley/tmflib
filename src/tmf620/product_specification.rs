//! Product Specification Module
//! 

use serde::{Deserialize,Serialize};
use uuid::Uuid;
use std::convert::Into;

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
        let href = format!("/{}/{}/{}",MOD_PATH,SPEC_PATH,id);
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

impl Into<ProductSpecificationRef> for ProductSpecification {
    fn into(self) -> ProductSpecificationRef {
        ProductSpecificationRef { id: self.id, href: self.href, name: self.name, version: self.version }
    }
}
/// Product Specification Characteristic Value Use
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct ProductSpecificationCharacteristicValueUse {}