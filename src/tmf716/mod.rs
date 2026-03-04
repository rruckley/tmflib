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
//! TMF716 Resource Reservation Management API Specification

pub use any::*;
pub use applicable_time_period::*;
pub use attachment_ref_or_value::*;
pub use cancel_resource_reservation::*;
pub use cancel_resource_reservation_create::*;
pub use cancel_resource_reservation_create_event::*;
pub use cancel_resource_reservation_create_event_payload::*;
pub use cancel_resource_reservation_information_required_event::*;
pub use cancel_resource_reservation_information_required_event_payload::*;
pub use cancel_resource_reservation_state_change_event::*;
pub use cancel_resource_reservation_state_change_event_payload::*;
pub use capacity::*;
pub use capacity_amount::*;
pub use capacity_ref::*;
pub use capacity_spec_ref::*;
pub use channel_ref::*;
pub use characteristic::*;
pub use characteristic_relationship::*;
pub use constraint_ref::*;
pub use event_subscription::*;
pub use event_subscription_input::*;
pub use feature::*;
pub use feature_relationship::*;
pub use note::*;
pub use place_ref_or_value::*;
pub use quantity::*;
pub use related_entity::*;
pub use related_party::*;
pub use related_party_ref::*;
pub use related_place_ref_or_value::*;
pub use reservation_item_action_type::*;
pub use reservation_item_state_type::*;
pub use reservation_state_type::*;
pub use resource_administrative_state_type::*;
pub use resource_operational_state_type::*;
pub use resource_ref_or_value::*;
pub use resource_relationship::*;
pub use resource_reservation::*;
pub use resource_reservation_attribute_value_change_event::*;
pub use resource_reservation_attribute_value_change_event_payload::*;
pub use resource_reservation_create::*;
pub use resource_reservation_create_event::*;
pub use resource_reservation_create_event_payload::*;
pub use resource_reservation_delete_event::*;
pub use resource_reservation_delete_event_payload::*;
pub use resource_reservation_information_required_event::*;
pub use resource_reservation_information_required_event_payload::*;
pub use resource_reservation_item::*;
pub use resource_reservation_ref::*;
pub use resource_reservation_state_change_event::*;
pub use resource_reservation_state_change_event_payload::*;
pub use resource_reservation_update::*;
pub use resource_specification_ref::*;
pub use resource_status_type::*;
pub use resource_usage_state_type::*;
pub use task_state_type::*;
pub use time_period::*;
mod any;
mod applicable_time_period;
mod attachment_ref_or_value;
mod cancel_resource_reservation;
mod cancel_resource_reservation_create;
mod cancel_resource_reservation_create_event;
mod cancel_resource_reservation_create_event_payload;
mod cancel_resource_reservation_information_required_event;
mod cancel_resource_reservation_information_required_event_payload;
mod cancel_resource_reservation_state_change_event;
mod cancel_resource_reservation_state_change_event_payload;
mod capacity;
mod capacity_amount;
mod capacity_ref;
mod capacity_spec_ref;
mod channel_ref;
mod characteristic;
mod characteristic_relationship;
mod constraint_ref;
mod event_subscription;
mod event_subscription_input;
mod feature;
mod feature_relationship;
mod note;
mod place_ref_or_value;
mod quantity;
mod related_entity;
mod related_party;
mod related_party_ref;
mod related_place_ref_or_value;
mod reservation_item_action_type;
mod reservation_item_state_type;
mod reservation_state_type;
mod resource_administrative_state_type;
mod resource_operational_state_type;
mod resource_ref_or_value;
mod resource_relationship;
mod resource_reservation;
mod resource_reservation_attribute_value_change_event;
mod resource_reservation_attribute_value_change_event_payload;
mod resource_reservation_create;
mod resource_reservation_create_event;
mod resource_reservation_create_event_payload;
mod resource_reservation_delete_event;
mod resource_reservation_delete_event_payload;
mod resource_reservation_information_required_event;
mod resource_reservation_information_required_event_payload;
mod resource_reservation_item;
mod resource_reservation_ref;
mod resource_reservation_state_change_event;
mod resource_reservation_state_change_event_payload;
mod resource_reservation_update;
mod resource_specification_ref;
mod resource_status_type;
mod resource_usage_state_type;
mod task_state_type;
mod time_period;
