// Copyright [2024] [Ryan Ruckley]

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! TMF Library
//! # Description
//! This library covers structures required to interact with various TMForum APIs.
//! It does not define amy persistence nor provide a REST interface (at this stage)
//! but simply provides definitions of all the schema and some helpful functions to create compliant objects
//! that can then be seriliased into or from JSON as required.
//! 
//! # Crate Features
//! 
//! ### API Version Features
//! 
//! By default this crate will compile v4 versions of APIs. 
//! * **v4**
//! This is the default version compiled
//! * **v5**
//! This flag can optionally be enabled to compile v5 APIs where available

#![warn(missing_docs)]

use std::str::FromStr;

use chrono::{Utc,Days};
use common::related_party::RelatedParty;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::common::note::Note;
use sha256::digest;
use hex::decode;
use base32::encode;

/// Primary path for the whole library, All paths generated will start with this.
pub const LIB_PATH: &str = "tmf-api";
/// Default code length used by [gen_code] if no length is supplied.
pub const CODE_DEFAULT_LENGTH : usize = 6;

/// Standard cardinality type for library
pub type Cardinality = u16;
/// Type alias for TimeStamps
pub type TimeStamp = String;
/// Type alias for DateTime
pub type DateTime = String;
/// Type alias for Uri
pub type Uri = String;

/// Standard TMF TimePeriod structure
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimePeriod {
    /// Start of time period
    pub start_date_time: TimeStamp,
    /// End of time period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<TimeStamp>,
}

impl TimePeriod {
    /// Create a time period of 30 days
    pub fn period_30days() -> TimePeriod {
        TimePeriod::period_days(30)
    }

    /// Calculate period `days` into the future
    pub fn period_days(days : u64) -> TimePeriod {
        let now = Utc::now() + Days::new(days);
        let time = chrono::DateTime::from_timestamp(now.timestamp(),0).unwrap();
        TimePeriod {
            end_date_time: Some(time.to_rfc3339()),
            ..Default::default()
        }
    }
    /// Return true if start time of TimePeriod is in the past.
    pub fn started(&self) -> bool {
        let now = Utc::now();
        let time = chrono::DateTime::from_timestamp(now.timestamp(),0).unwrap();
        false
    }
    /// Return true if the finish time is set and is in the past
    pub fn finished(&self) -> bool {
        match &self.end_date_time {
            Some(f) => {
                let now = Utc::now();
                let time = chrono::DateTime::from_timestamp(now.timestamp(),0).unwrap();
                let dt = chrono::DateTime::parse_from_rfc3339(&self.start_date_time).unwrap();
        
                let s= &self.start_date_time.as_str();
                true
            },
            None => false
        }
    }
}

impl Default for TimePeriod {
    fn default() -> Self {
        let now = Utc::now();
        let time = chrono::DateTime::from_timestamp(now.timestamp(),0).unwrap();
        TimePeriod {
            start_date_time : time.to_rfc3339(),
            end_date_time: None,
        }
    }
}

/// Generate a cryptographic code for use in API calls.
/// 
/// Currently used by:
/// - [`crate::tmf632::individual_v4::Individual`]
/// - [`crate::tmf632::organization_v4::Organization`]
/// - [`crate::tmf629::customer::Customer`]
/// - [`crate::tmf674::geographic_site_v4::GeographicSite`]
/// # Returns
///  Returns tuple of the generated code and the Base32 Hash used to form the code.
/// # Algorithm
/// This function takes the supplied inputs (name, id , offset) and generates a cryptographic hash which is then
/// output as a Base32 hash. Each Base32 digit represents 5 bits of binary data, so 6 digits provides 30 bits of 
/// data or around 1 Billion possible codes.
/// # Example
/// ```
/// use tmflib::gen_code;
/// let (code,hash) = gen_code("John Q. Smith".to_string(),"USER123".to_string(),None,Some("U-".to_string()),None);
/// ```
pub fn gen_code(name : String, id : String, offset : Option<u32>, prefix : Option<String>,length : Option<usize>) -> (String,String) {
    let hash_input = format!("{}:{}:{}",name,id,offset.unwrap_or_default());
    let sha = digest(hash_input);
    let hex = decode(sha);
    let base32 = encode(base32::Alphabet::Rfc4648 { padding: false }, hex.unwrap().as_ref());
    let sha_slice = base32.as_str()[..length.unwrap_or(CODE_DEFAULT_LENGTH)].to_string().to_ascii_uppercase();
    (format!("{}{}",prefix.unwrap_or_default(),sha_slice),base32)
}

/// Trait indicating a TMF struct has and id and corresponding href field
pub trait HasId : Default {
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
    /// Get the class of this object. This is also used to form part of the URL via generate_href()
    fn get_class() -> String;
    /// Get Class HREF, this represents the generate path to the class.
    fn get_class_href() -> String;
    /// Get the module path
    fn get_mod_path() -> String;
    /// Set the id on the object, also triggers generate_href().
    fn set_id(&mut self, id : impl Into<String>);
    /// Create a new instance of a TMF object that has id and href fields.
    /// # Example
    /// ```
    /// # use crate::tmflib::tmf629::customer::Customer;
    /// # use crate::tmflib::HasId;
    /// let offering = Customer::create();
    /// ```` 
    fn create() -> Self {
        // Create default instance
        let mut item = Self::default();
        // Generate unique id and href
        item.generate_id();
        item
    }
}

/// Trait indicating a TMF sturct has a last_update or similar timestamp field.
pub trait HasLastUpdate : HasId {
    /// Geneate a timestamp for now(), useful for updating last_updated fields
    fn get_timestamp() -> String {
        let now = Utc::now();
        let time = chrono::DateTime::from_timestamp(now.timestamp(),0).unwrap();
        time.to_string()
    }

    /// Store a timestamp into last_update field (if available)
    fn set_last_update(&mut self, time : impl Into<String>);

    /// Create a new TMF object, also set last_update field to now()
    fn create_with_time() -> Self {
        // Create default instance
        let mut item = Self::default();
        item.generate_id();
        item.set_last_update(Self::get_timestamp());
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
    /// Return true as follows:
    /// - If no end is set and the start is in the past return true.
    /// - If end is set and start is in the past and end is in the future, return true.
    /// - Otherwise return false.
    fn is_valid(&self) -> bool;
}

/// Does an object have a name field?
pub trait HasName : HasId {
    /// Return name of object
    fn get_name(&self) -> String;
    /// Match against the name
    fn find(&self, pattern : &str) -> bool {
        self.get_name().contains(pattern.trim())
    }
    /// Set the name, trimming any whitespace
    fn set_name(&mut self, name : impl Into<String>);
}

/// Trait for classes with notes
pub trait HasNote : HasId {
    /// Get a specific note if it exists
    fn get_note(&self, idx : usize) -> Option<&Note>;
    /// Add a new note
    fn add_note(&mut self, note : Note);
    /// Remove a note by index
    fn remove_note(&mut self, idx: usize) -> Result<Note,String>;
}

/// Trait for classes with Related Parties
pub trait HasRelatedParty : HasId {
    /// Get a specific party by index
    fn get_party(&self, idx : usize ) -> Option<&RelatedParty>;
    /// Add a new party
    fn add_party(&mut self, party : RelatedParty);
    /// Remote a party
    fn remove_party(&mut self, idx : usize) -> Result<RelatedParty,String>;
}

/// Trait for generating an event
pub trait TMFEvent<T> : HasId + HasName {
    /// Geneate container for an TMF payload to be used in an event
    fn event(&self) -> T;
}

/// Common Modules
pub mod common;
/// Product Catalogue
#[cfg(any(feature = "tmf620-v4" , feature = "tmf620-v5"))]
pub mod tmf620;
/// Product Order
#[cfg(any(feature = "tmf622-v4" , feature = "tmf622-v5"))]
pub mod tmf622;
/// Customer
#[cfg(any(feature = "tmf629-v4" , feature = "tmf629-v5"))]
pub mod tmf629;
/// Party
#[cfg(any(feature = "tmf632-v4" , feature = "tmf632-v5"))]
pub mod tmf632;
pub mod tmf633;
pub mod tmf634;
pub mod tmf637;
pub mod tmf638;
pub mod tmf639;
pub mod tmf641;
pub mod tmf646;
pub mod tmf648;
pub mod tmf651;
pub mod tmf653;
pub mod tmf663;
pub mod tmf666;
#[cfg(any(feature = "tmf667-v4" , feature = "tmf667-v5"))]
pub mod tmf667;
pub mod tmf669;
pub mod tmf672;
pub mod tmf673;
#[cfg(any(feature = "tmf674-v4" , feature = "tmf674-v5"))]
pub mod tmf674;
#[cfg(any(feature = "tmf678-v4" , feature = "tmf678-v5"))]
pub mod tmf678;
pub mod tmf679;
pub mod tmf681;
#[cfg(any(feature = "tmf699-v4" , feature = "tmf699-v5"))]
pub mod tmf699;
pub mod tmf700;
pub mod tmf724;
pub mod tmf760;

#[cfg(test)]
mod test {
    use super::gen_code;
    const CODE : &str = "T-DXQR65";
    const HASH : &str = "DXQR656VE3FIKEZZWJX6C3WC27NSRTJVMYR7ILA5XNDLSJXQPDVQ";
    #[test]
    fn test_gen_code() {
        // Generate a code with a known hash
        let (code,hash) = gen_code("NAME".into(),"CODE".into(),None,Some("T-".into()),None);

        assert_eq!(code,CODE.to_string());
        assert_eq!(hash,HASH.to_string());
    }
}