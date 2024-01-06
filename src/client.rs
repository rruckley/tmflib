//! Client Module
//! 
//! Provides HTTP interface into a downstream TMF compliant source
//! 

/// HTTP Client to a TMF compliant downstream system
#[derive(Debug,Default,Clone)]
pub struct TMFClient {
    host : String,
}

impl TMFClient {
    /// Create a new client instance against the given URL
    pub fn new(host : &str) -> TMFClient {
        TMFClient {
            host : host.to_string(),
        }
    }
}