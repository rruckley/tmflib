//! Characteristic Specification Module

use serde::{Deserialize,Serialize};


/// Service Specification Characteristics
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct CharacteristicSpecification {
    /// Can this characteristic be configured?
    pub configurable: Option<bool>,
    /// Unique Id
    pub id: Option<String>,
    /// Name
    pub name: Option<String>,
    /// Minimum instance
    pub min_cardinality: u16,
    /// Maximum instance
    pub max_cardinality: u16,
}

impl CharacteristicSpecification {
    /// Constructor
    pub fn new(name : impl Into<String>) -> CharacteristicSpecification {
        let mut cs = CharacteristicSpecification::default();
        cs.name = Some(name.into());
        cs.max_cardinality = 1;
        cs
    }
}