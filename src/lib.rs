// Copyright [2025] [Ryan Ruckley]

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
//! This library covers data structures required to interact with various TMForum APIs.
//! It does not define any persistence nor provide a REST interface (at this stage)
//! but simply provides definitions of all the schema and helpful functions and traits to create and maniuplate compliant objects
//! that can then be seriliased into or from JSON as required.
//! ### API Version Features
//! By default this crate will compile v4 versions of APIs.
//! * **build-V4**
//!   This is the default version compiled
//! * **build-V5**
//!
//! This flag can be enabled to compile v5 APIs where available, mutually exclusive with build-V4.

//! ### Common Feature ###
//! Within the library is a set of common modules. These modules refer to other TMF modules and thus all
//! modules referenced by the common module are included under this feature.
//! Specifically:
//! - [tmf620]
//! - [tmf629]
//! - [tmf632]
//! - [tmf666]
//! - [tmf667]
//! - [tmf669]
//! - [tmf674]

//! ### ODA Component Features
//!
//! All [ODA Component](https://www.tmforum.org/oda/directory/components-map) identifiers, e.g. TMFC001 have been mapped onto features to enable building the library
//! to support a specific component.

//! *NB: For components that dont' have any defined APIs, a common set of APIs are included under the **[common]** feature*

#![warn(missing_docs)]
// #![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::private_doc_tests)]

use crate::common::note::Note;
use base32::encode;
use chrono::{Days, Utc};
use common::{attachment::AttachmentRefOrValue, related_party::RelatedParty, tmf_error::TMFError};
use hex::decode;
use serde::{Deserialize, Serialize};
use sha256::digest;
use uuid::Uuid;

/// Primary path for the whole library, All paths generated will start with this.
pub const LIB_PATH: &str = "tmf-api";
/// Default code length used by [gen_code] if no length is supplied.
pub const CODE_DEFAULT_LENGTH: usize = 6;

/// Standard cardinality type for library
pub type Cardinality = u16;
/// Type alias for TimeStamps
pub type TimeStamp = String;
/// Type alias for DateTime
pub type DateTime = String;
/// Type alias for Uri
pub type Uri = String;
/// Priority Type
pub type Priority = u16;

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
    pub fn period_days(days: u64) -> TimePeriod {
        let now = Utc::now() + Days::new(days);
        let time =
            chrono::DateTime::from_timestamp(now.timestamp(), 0).expect("Invalid now() output");
        TimePeriod {
            end_date_time: Some(time.to_rfc3339()),
            ..Default::default()
        }
    }
    /// Return true if start time of TimePeriod is in the past.
    pub fn started(&self) -> bool {
        let now = Utc::now();

        let start = chrono::DateTime::parse_from_rfc3339(&self.start_date_time)
            .expect("Could not start parse time from now()");
        // Start is in the past, return true
        if start < now {
            return true;
        }
        false
    }
    /// Return true if the finish time is set and is in the past
    pub fn finished(&self) -> bool {
        match &self.end_date_time {
            Some(f) => {
                let now = Utc::now();
                let finish = chrono::DateTime::parse_from_rfc3339(f)
                    .expect("Could not parse finish time from now()");
                if finish < now {
                    return true;
                }
                false
            }
            None => false,
        }
    }
}

impl Default for TimePeriod {
    fn default() -> Self {
        let now = Utc::now();
        let time =
            chrono::DateTime::from_timestamp(now.timestamp(), 0).expect("Invalid input timestamp");
        TimePeriod {
            start_date_time: time.to_rfc3339(),
            end_date_time: None,
        }
    }
}

impl From<DateTime> for TimePeriod {
    fn from(value: TimeStamp) -> Self {
        TimePeriod {
            start_date_time: value.clone(),
            end_date_time: None,
        }
    }
}

/// Basic Amount / Unit quantity structure
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Quantity {
    /// How much?
    pub amount: f64,
    /// What type?
    pub units: String,
}

impl Quantity {
    /// Create a simple Quantity representing weight in kg
    /// # Example
    /// ```
    /// use tmflib::Quantity;
    /// let weight = Quantity::kg(10.5);
    /// assert_eq!(weight.amount,10.5);
    /// ```
    pub fn kg(amount: f64) -> Quantity {
        Quantity {
            amount,
            units: "kg".to_string(),
        }
    }
    /// Shortcut functions to set carton quantity and associated units.
    pub fn cartons(amount: f64) -> Quantity {
        Quantity {
            amount,
            units: "cartons".to_string(),
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
/// assert_eq!(code,"U-SP7E6E".to_string());
/// ```
pub fn gen_code(
    name: String,
    id: String,
    offset: Option<u32>,
    prefix: Option<String>,
    length: Option<usize>,
) -> (String, String) {
    let hash_input = format!("{}:{}:{}", name, id, offset.unwrap_or_default());
    let sha = digest(hash_input);
    let hex = decode(sha);
    let base32 = encode(
        base32::Alphabet::Rfc4648 { padding: false },
        hex.expect("Could not parse HEX string from digest()")
            .as_ref(),
    );
    let sha_slice = base32.as_str()[..length.unwrap_or(CODE_DEFAULT_LENGTH)]
        .to_string()
        .to_ascii_uppercase();
    (
        format!("{}{}", prefix.unwrap_or_default(), sha_slice),
        base32,
    )
}

/// Return type for a serde_json Value
pub fn serde_value_to_type(value: &serde_json::Value) -> &str {
    match value {
        serde_json::Value::Null => "Null",
        serde_json::Value::Bool(_) => "Bool",
        serde_json::Value::Number(_) => "Number",
        serde_json::Value::String(_) => "String",
        serde_json::Value::Array(_) => "Array",
        serde_json::Value::Object(_) => "Object",
    }
}

/// Perform an safe insert operation on a optional vector of TMF objects
/// # Actions
/// - If Option is Some(v) then item is inserted into v
/// - If Option is None then a new Vec is created with item as single entry
pub fn vec_insert<T>(ov: &mut Option<Vec<T>>, item: T) {
    match ov.as_mut() {
        Some(v) => {
            v.push(item);
        }
        None => {
            let _old_i = ov.replace(vec![item]);
        }
    }
}

/// Trait indicating a TMF struct has and id and corresponding href field
pub trait HasId: Default {
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
    fn set_id(&mut self, id: impl Into<String>);
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
    /// Builder pattern to set id on create()
    /// NB: This can be used to set an explicit id on create instead of auto-generate via `[create`]
    fn id(self, id: impl Into<String>) -> Self;
}

/// Trait indicating a TMF sturct has a last_update or similar timestamp field.
pub trait HasLastUpdate: HasId {
    /// Geneate a timestamp for now(), useful for updating last_updated fields
    fn get_timestamp() -> String {
        let now = Utc::now();
        let time = chrono::DateTime::from_timestamp(now.timestamp(), 0)
            .expect("Invalid timestamp from now()");
        time.to_string()
    }

    /// Store a timestamp into last_update field (if available)
    fn set_last_update(&mut self, time: impl Into<String>);

    /// Create a new TMF object, also set last_update field to now()
    fn create_with_time() -> Self {
        // Create default instance
        let mut item = Self::default();
        item.generate_id();
        item.set_last_update(Self::get_timestamp());
        item
    }

    /// Builder pattern for setting lastUpdate on create
    /// If time is None, current time is used via ['get_timestamp()']
    fn last_update(self, time: Option<String>) -> Self;
}

/// Trait for classes with a valid_for object covering validity periods.
pub trait HasValidity {
    /// Set the validity by passing in a [`TimePeriod`]
    fn set_validity(&mut self, validity: TimePeriod);
    /// Get the current validity, might not be set
    fn get_validity(&self) -> Option<TimePeriod>;
    /// Get the start of the validity period, might not be set
    fn get_validity_start(&self) -> Option<TimeStamp>;
    /// Get the end of the validity period, might not be set
    fn get_validity_end(&self) -> Option<TimeStamp>;
    /// Set only the start of the validity period, returns updated [`TimePeriod`]
    fn set_validity_start(&mut self, start: TimeStamp) -> TimePeriod;
    /// Set only the end of the validty period, returns updated [`TimePeriod`]
    fn set_validity_end(&mut self, end: TimeStamp) -> TimePeriod;
    /// Return true as follows:
    /// - If no end is set and the start is in the past return true.
    /// - If end is set and start is in the past and end is in the future, return true.
    /// - Otherwise return false.
    fn is_valid(&self) -> bool;
    /// Builder pattern function to add validity on create
    fn validity(self, validity: TimePeriod) -> Self;
}

/// Does an object have a name field?
pub trait HasName: HasId {
    /// Return name of object
    fn get_name(&self) -> String;
    /// Match against the name
    fn find(&self, pattern: &str) -> bool {
        self.get_name().contains(pattern.trim())
    }
    /// Set the name, trimming any whitespace
    fn set_name(&mut self, name: impl Into<String>);
    /// Builder pattern to set name on create, usually coverered by new()
    fn name(self, name: impl Into<String>) -> Self;
}

/// Trait for classes with notes
pub trait HasNote: HasId {
    /// Get a specific note if it exists
    fn get_note(&self, idx: usize) -> Option<&Note>;
    /// Add a new note
    fn add_note(&mut self, note: Note);
    /// Remove a note by index
    fn remove_note(&mut self, idx: usize) -> Result<Note, TMFError>;
    /// Builder pattern to add note on create
    fn note(self, note: Note) -> Self;
}

/// Trait for classes with Related Parties
pub trait HasRelatedParty: HasId {
    /// Get a specific party by index
    fn get_party(&self, idx: usize) -> Option<&RelatedParty>;
    /// Add a new party
    fn add_party(&mut self, party: RelatedParty);
    /// Remote a party
    fn remove_party(&mut self, idx: usize) -> Result<RelatedParty, TMFError>;
    /// Get a list of RelatedParty entries by role
    fn get_by_role(&self, role: String) -> Option<Vec<&RelatedParty>>;
    /// Builder pattern to add a party on create
    fn party(self, party: RelatedParty) -> Self;
}

/// Trait for generating an event
pub trait TMFEvent<T>: HasId {
    /// Geneate container for an TMF payload to be used in an event
    fn event(&self) -> T;
}

/// Struct has Attachments
pub trait HasAttachment {
    /// Add an attachment, Base64 encoding the data
    /// vec[] will be created as required.
    fn add(&mut self, attachment: &AttachmentRefOrValue);
    /// Find an attachement based on matching string against filename
    fn position(&self, name: impl Into<String>) -> Option<usize>;
    /// Retrieve an attachment based on name
    fn find(&self, name: impl Into<String>) -> Option<&AttachmentRefOrValue>;
    /// Get a specific attachment returing value
    fn get(&self, position: usize) -> Option<AttachmentRefOrValue>;
    /// Remove an attachment at a particular position
    fn remove(&mut self, position: usize) -> Option<AttachmentRefOrValue>;
    /// builder pattern function to add attachment on create
    fn attachment(self, attachment: AttachmentRefOrValue) -> Self;
}

/// Trait for managing a description field. Description field must be defined as `Option<String>`
pub trait HasDescription {
    /// Builder pattern function to set the description on object creation
    fn description(self, description: impl Into<String>) -> Self;
    /// Get the description by cloning it if set, returns empty string otherwise.
    fn get_description(&self) -> String;
    /// Update the description by inserting a new value into the Option.
    /// Returns the old value if set otherwise None.
    fn set_description(&mut self, description: impl Into<String>) -> Option<String>;
}

/// Trait for objects that have a reference version object or can be converted into an EneityRef
pub trait HasReference: HasId + HasName {
    /// Reference type assocaited with Self.
    type RefType: Serialize;
    /// Get object as an EntityRef
    fn as_entity_ref(&self) -> crate::common::related_entity::RelatedEntity {
        crate::common::related_entity::RelatedEntity {
            id: self.get_id(),
            href: self.get_href(),
            name: self.get_name(),
            referred_type: Self::get_class(),
            role: None,
        }
    }

    /// Return a reference version of an object, if none exists, return None.
    fn as_ref(&self) -> Option<Self::RefType> {
        None
    }
}

pub mod common;
#[cfg(feature = "tmf620")]
pub mod tmf620;
#[cfg(feature = "tmf621")]
pub mod tmf621;
#[cfg(feature = "tmf622")]
pub mod tmf622;
#[cfg(feature = "tmf629")]
pub mod tmf629;
#[cfg(feature = "tmf632")]
pub mod tmf632;
#[cfg(feature = "tmf633")]
pub mod tmf633;
#[cfg(feature = "tmf634")]
pub mod tmf634;
#[cfg(feature = "tmf637")]
pub mod tmf637;
#[cfg(feature = "tmf638")]
pub mod tmf638;
#[cfg(feature = "tmf639")]
pub mod tmf639;
#[cfg(feature = "tmf641")]
pub mod tmf641;
#[cfg(feature = "tmf645")]
pub mod tmf645;
#[cfg(feature = "tmf646")]
pub mod tmf646;
#[cfg(feature = "tmf648")]
pub mod tmf648;
#[cfg(feature = "tmf651")]
pub mod tmf651;
#[cfg(feature = "tmf653")]
pub mod tmf653;
#[cfg(feature = "tmf663")]
pub mod tmf663;
#[cfg(feature = "tmf664")]
pub mod tmf664;
#[cfg(feature = "tmf666")]
pub mod tmf666;
#[cfg(feature = "tmf667")]
pub mod tmf667;
#[cfg(feature = "tmf669")]
pub mod tmf669;
#[cfg(feature = "tmf672")]
pub mod tmf672;
#[cfg(feature = "tmf673")]
pub mod tmf673;
#[cfg(feature = "tmf674")]
pub mod tmf674;
#[cfg(feature = "tmf678")]
pub mod tmf678;
#[cfg(feature = "tmf679")]
pub mod tmf679;
#[cfg(feature = "tmf680")]
pub mod tmf680;
#[cfg(feature = "tmf681")]
pub mod tmf681;
#[cfg(feature = "tmf687")]
pub mod tmf687;
#[cfg(feature = "tmf696")]
pub mod tmf696;
#[cfg(feature = "tmf697")]
pub mod tmf697;
#[cfg(feature = "tmf699")]
pub mod tmf699;
#[cfg(feature = "tmf700")]
pub mod tmf700;
#[cfg(feature = "tmf724")]
pub mod tmf724;
#[cfg(feature = "tmf760")]
pub mod tmf760;
#[cfg(feature = "tmf921")]
pub mod tmf921;

#[cfg(test)]
mod test {
    use crate::{HasName, Quantity, TimePeriod};

    use super::gen_code;
    use super::vec_insert;
    use crate::common::related_party::RelatedParty;
    #[cfg(all(feature = "tmf632", feature = "build-V4"))]
    use crate::tmf632::organization_v4::Organization;
    #[cfg(all(feature = "tmf632", feature = "build-V5"))]
    use crate::tmf632::organization_v5::Organization;

    const CODE: &str = "T-DXQR65";
    const HASH: &str = "DXQR656VE3FIKEZZWJX6C3WC27NSRTJVMYR7ILA5XNDLSJXQPDVQ";
    const CARTON_QTY: f64 = 12.34;
    const ORG_NAME: &str = "Organisation";
    const QUANTITY_JSON: &str = "{
        \"amount\" : 12.34,
        \"units\" : \"units\"
    }";
    const PERIOD_JSON: &str = "{
        \"startDateTime\" : \"2024-07-29T23:07:57Z\"
    }";
    #[test]
    fn test_gen_code() {
        // Generate a code with a known hash
        let (code, hash) = gen_code("NAME".into(), "CODE".into(), None, Some("T-".into()), None);

        assert_eq!(code, CODE.to_string());
        assert_eq!(hash, HASH.to_string());
    }

    #[test]
    fn test_quantity_kg() {
        let quantity = Quantity::kg(10.5);

        assert_eq!(quantity.amount, 10.5);
        assert_eq!(quantity.units, "kg".to_string());
    }

    #[test]
    fn test_timeperiod_30days() {
        let days = TimePeriod::period_30days();

        assert_eq!(days.started(), true);
        assert_eq!(days.finished(), false);
    }

    #[test]
    fn test_timeperiod_default() {
        let default_period = TimePeriod::default();

        assert_eq!(default_period.started(), true);
        assert_eq!(default_period.end_date_time.is_none(), true);
    }

    #[test]
    fn test_timeperiod_finished() {
        let mut finished = TimePeriod::default();

        // At this point, end_date_time is not set, should  return !finished().
        assert_eq!(finished.finished(), false);
        // Assumption is some small period of time has elapsed since setting start_time so that
        // start time will be in the past.
        finished.end_date_time = Some(finished.start_date_time.clone());

        assert_eq!(finished.finished(), true);
    }

    #[test]
    fn test_quantity_cartons() {
        let quantity = Quantity::cartons(CARTON_QTY);

        assert_eq!(quantity.amount, CARTON_QTY);
        assert_eq!(quantity.units.as_str(), "cartons");
    }

    #[test]
    fn test_hasname_find() {
        let cust = Organization::new(ORG_NAME);

        let find_match = cust.find("Org");

        assert_eq!(find_match, true);
    }

    #[test]
    fn test_quantity_deserialize() {
        let quantity: Quantity =
            serde_json::from_str(QUANTITY_JSON).expect("Could not parse Quantity JSON");

        assert_eq!(quantity.amount, 12.34);
        assert_eq!(quantity.units.as_str(), "units");
    }

    #[test]
    fn test_timeperiod_deserialize() {
        let period: TimePeriod =
            serde_json::from_str(PERIOD_JSON).expect("Could not parse Period JSON");

        assert_eq!(period.start_date_time.as_str(), "2024-07-29T23:07:57Z");
        assert_eq!(period.end_date_time.is_none(), true);
    }

    #[test]
    fn test_timeperiod_not_started() {
        let old_period = TimePeriod::period_30days();

        let mut new_period = TimePeriod::default();
        new_period.start_date_time = old_period
            .end_date_time
            .expect("perdio_30days() did not set end date")
            .clone();

        assert_eq!(new_period.started(), false);
    }

    #[test]
    fn test_vecinsert_none() {
        let rp = RelatedParty::default();
        let mut ov: Option<Vec<RelatedParty>> = None;

        vec_insert(&mut ov, rp);

        assert_eq!(ov.is_some(), true);
        assert_eq!(ov.unwrap().len(), 1);
    }

    #[test]
    fn test_vecinsert_some() {
        let mut rp = RelatedParty::default();
        let _prev = rp.name.insert(String::from("one"));
        let mut ov: Option<Vec<RelatedParty>> = Some(vec![rp]);

        let rp2 = RelatedParty::default();

        vec_insert(&mut ov, rp2);

        assert_eq!(ov.is_some(), true);
        assert_eq!(ov.unwrap().len(), 2);
    }
}
