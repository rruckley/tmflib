//! TMF629 Customer Management Module
//!
//!
//!

use super::customer::Customer;

use serde::{Deserialize, Serialize};

/// Management layer for TMF629
/// # Warning
/// This will be moved into platypus
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct TMF629CustomerManagement {
    customers: Vec<Customer>,
}

impl TMF629CustomerManagement {
    /// Create a new instance of the management liayer
    /// # Warning
    /// This will be moved into playpus to handle state
    pub fn new() -> TMF629CustomerManagement {
        TMF629CustomerManagement { customers: vec![] }
    }
}
