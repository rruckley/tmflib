// Copyright 2023-2023 Ryan Ruckley.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

//! TMF Library
//! # Description
//! This library covers objects required to interact with various TMF defined APIs. 
//! It does not define how to interact with those APIs nor provide a REST interface (at this stage)
//! but simply provides definitions of all the schema and some helpful functions to create compliant objects
//! that can then be seriliased into or from JSON as required.

#![warn(missing_docs)]

use chrono::naive::NaiveDateTime;
use chrono::Utc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::default::Default;

/// Primary path for the whole library
pub const LIB_PATH: &str = "tmf-api";

/// Standard cardinality type for library
pub type Cardinality = u16;
/// Type alias for TimeStamps
pub type TimeStamp = String;
/// Type alias for DateTime
pub type DateTime = String;

/// Standard TMF TimePeriod structure
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimePeriod {
    /// Start of time period
    pub start_date_time: TimeStamp,
    /// End of time period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<TimeStamp>,
}

impl Default for TimePeriod {
    fn default() -> Self {
        let now = Utc::now();
        let time = NaiveDateTime::from_timestamp_opt(now.timestamp(), 0).unwrap();
        TimePeriod { start_date_time: time.to_string(), end_date_time: None }
    }
}

/// Trait indicating a TMF struct has and id and corresponding href field
pub trait HasId {
    /// Get a new UUID in simple format (no seperators)
    fn get_uuid() -> String {
        // Using simple format as SurrealDB doesn't like dashes in standard format.
        Uuid::new_v4().simple().to_string()
    }
    /// Generate and store a new ID. This will also regenerated the HREF field via generate_href()
    fn generate_id(&mut self);
    /// Generate a new HTML reference.
    /// # Details
    /// This is usually triggered directly from generate_id() but can be manually triggered.
    fn generate_href(&mut self);
    /// Extract the id of this object into a new String
    fn get_id(&self) -> String;
    /// Extract the HREF of this object into a new String
    fn get_href(&self) -> String;
    /// Get the class of this object
    fn get_class() -> String;
    /// Get Class HREF
    fn get_class_href() -> String;
}

/// Trait to create TMF structs that have the HasId trait
pub trait CreateTMF<T : Default + HasId> {
    /// Create a new instance of a TMF object that has id and href fields.
    /// # Example
    /// ```
    /// # use crate::tmflib::tmf629::customer::Customer;
    /// use crate::tmflib::CreateTMF;
    /// let offering = Customer::create();
    /// ```` 
    fn create() -> T {
        // Create default instance
        let mut item = T::default();
        // Generate unique id and href
        item.generate_id();
        item
    }
}

/// Trait indicating a TMF sturct has a last_update or similar timestamp field.
pub trait HasLastUpdate {
    /// Geneate a timestamp for now(), useful for updating last_updated fields
    fn get_timestamp() -> String {
        let now = Utc::now();
        let time = NaiveDateTime::from_timestamp_opt(now.timestamp(), 0).unwrap();
        time.to_string()
    }

    /// Store a timestamp into last_update field (if available)
    fn set_last_update(&mut self, time : impl Into<String>);
}

/// Trait to create a TMF struct including a timestamp field
pub trait CreateTMFWithTime<T : Default + HasId + HasLastUpdate> {
    /// Create a new TMF object, also set last_update field to now()
    fn create_with_time() -> T {
        // Create default instance
        let mut item = T::default();
        item.generate_id();
        item.set_last_update(T::get_timestamp());
        item
    }
}

/// Trait for classes with a valid_for object covering validity periods.
pub trait HasValidity {
    /// Set the validity by passing in a [`TimePeriod`]
    fn set_validity(&mut self, validity : TimePeriod);
    /// Get the current validity, might not be set
    fn get_validity(&self) -> Option<TimePeriod>;
    /// Get the start of the validity period, might not be set
    fn get_validity_start(&self) -> Option<TimeStamp>;
    /// Get the end of the validity period, might not be set
    fn get_validity_end(&self) -> Option<TimeStamp>;
    /// Set only the start of the validity period, returns updated [`TimePeriod`]
    fn set_validity_start(&mut self, start : TimeStamp) -> TimePeriod;
    /// Set only the end of the validty period, returns updated [`TimePeriod`]
    fn set_validity_end(&mut self, end : TimeStamp) -> TimePeriod;
}

/// Does an object have a name field?
pub trait HasName : HasId {
    /// Return name of object
    fn get_name(&self) -> String;
    /// Match against the name
    fn find(&self, pattern : &str) -> bool {
        self.get_name().contains(pattern.trim())
    }
}

/// Common Modules
pub mod common;
/// Product Catalogue
pub mod tmf620;
/// Product Order
pub mod tmf622;
/// Customer
pub mod tmf629;
/// Party
pub mod tmf632;
/// Service Catalog
pub mod tmf633;
/// Resource Catalog
pub mod tmf634;
/// Product Inventory
pub mod tmf637;
/// Service Inventory
pub mod tmf638;
/// Resource Inventory
pub mod tmf639;
/// Service Order
pub mod tmf641;
/// Appointment
pub mod tmf646;
/// Quote
pub mod tmf648;
/// Agreement
pub mod tmf651;
/// Service Test
pub mod tmf653;
/// Shopping Cart
pub mod tmf663;
/// Account
pub mod tmf666;
/// Party Role
pub mod tmf669;
/// Geographic Address
pub mod tmf673;
/// Geographic Site
pub mod tmf674;
/// Product Offering Qualification
pub mod tmf679;
/// Sales Management
pub mod tmf699;
/// Shipping Order [Pre-Prod]
pub mod tmf700;
/// Product Configuration
pub mod tmf760;
/// Cost Management
#[cfg(feature = "tmf7xx-v5")]
pub mod tmf7xx;