// Copyright [2026] [Ryan Ruckley]

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! Module for TMF642 - Alarm Management

pub use ack_alarm::*;
pub use addressable::*;
pub use alarm::*;
pub use alarm_ref::*;
pub use alarm_ref_mvo::*;
pub use alarm_ref_or_value::*;
pub use alarm_type::*;
pub use alarmed_object_ref::*;
pub use alarmed_object_ref_mvo::*;
pub use characteristic::*;
pub use characteristic_relationship::*;
pub use clear_alarm::*;
pub use comment::*;
pub use comment_alarm::*;
pub use comment_mvo::*;
pub use crossed_threshold_information::*;
pub use crossed_threshold_information_mvo::*;
pub use extensible::*;
pub use external_identifier::*;
pub use external_identifier_fvo::*;
pub use external_identifier_mvo::*;
pub use group_alarm::*;
pub use hub::*;
pub use party_ref_or_party_role_ref::*;
pub use perceived_severity::*;
pub use place::*;
pub use place_fvo::*;
pub use place_mvo::*;
pub use related_party_ref_or_party_role_ref::*;
pub use related_place::*;
pub use related_place_fvo::*;
pub use related_place_mvo::*;
pub use service_ref::*;
pub use service_ref_mvo::*;
pub use threshold_ref::*;
pub use threshold_ref_mvo::*;
pub use un_ack_alarm::*;
pub use un_group_alarm::*;
mod ack_alarm;
mod addressable;
pub mod alarm;
mod alarm_ref;
pub mod alarm_ref_mvo;
mod alarm_ref_or_value;
pub mod alarm_type;
pub mod alarmed_object_ref;
mod alarmed_object_ref_mvo;
mod characteristic;
mod characteristic_relationship;
mod clear_alarm;
mod comment;
mod comment_alarm;
mod comment_mvo;
mod crossed_threshold_information;
mod crossed_threshold_information_mvo;
mod extensible;
mod external_identifier;
mod external_identifier_fvo;
mod external_identifier_mvo;
mod group_alarm;
mod hub;
mod party_ref_or_party_role_ref;
pub mod perceived_severity;
pub mod place;
mod place_fvo;
mod place_mvo;
mod related_party_ref_or_party_role_ref;
mod related_place;
mod related_place_fvo;
mod related_place_mvo;
mod service_ref;
mod service_ref_mvo;
mod threshold_ref;
mod threshold_ref_mvo;
mod un_ack_alarm;
mod un_group_alarm;

const MOD_PATH: &str = "alarmManagement/v5";
