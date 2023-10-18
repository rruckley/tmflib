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

#![warn(missing_docs)]

use chrono::naive::NaiveDateTime;
use chrono::Utc;
use uuid::Uuid;

/// Primary path for the whole library
pub const LIB_PATH: &str = "tmflib";

/// Trait indicating a TMF struct has and id and corresponding href field
pub trait HasId {
    /// Get a new UUID in simple format
    fn get_uuid() -> String {
        // Using simple format as SurrealDB doesn't like dashes in standard format.
        Uuid::new_v4().as_simple().to_string()
    }
    /// Generate and store a new ID
    fn generate_id(&mut self);
    /// Generate a new HTML reference.
    /// # Details
    /// This is usually triggered directly from generate_id() but can be manually triggered.
    fn generate_href(&mut self);
    /// Extract the id of this object into a new String
    fn get_id(&self) -> String;
    /// Extract the HREF of this object into a new String
    fn get_href(&self) -> String;
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
    fn set_last_update(&mut self, time : String);
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

pub mod common;
pub mod tmf620;
pub mod tmf622;
pub mod tmf629;
pub mod tmf632;
pub mod tmf633;
pub mod tmf634;
pub mod tmf637;
pub mod tmf639;
pub mod tmf638;
pub mod tmf641;
pub mod tmf646;
pub mod tmf648;
pub mod tmf669;
pub mod tmf673;
pub mod tmf674;
pub mod tmf679;
pub mod tmf700;
pub mod tmf760;