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

//! TMF 671 - Promotion Management

pub use addressable::*;
pub use characteristic::*;
pub use characteristic_relationship::*;
pub use entity_ref::*;
pub use entity_ref_fvo::*;
pub use entity_ref_mvo::*;
pub use extensible::*;
pub use hub::*;
pub use party_ref_or_party_role_ref::*;
pub use promotion::*;
pub use promotion_action::*;
pub use promotion_action_fvo::*;
pub use promotion_action_mvo::*;
pub use promotion_criteria::*;
pub use promotion_criteria_group::*;
pub use promotion_criteria_group_fvo::*;
pub use promotion_criteria_group_mvo::*;
pub use promotion_criteria_fvo::*;
pub use promotion_criteria_mvo::*;
pub use promotion_pattern::*;
pub use promotion_pattern_fvo::*;
pub use promotion_pattern_mvo::*;
pub use related_party_ref_or_party_role_ref::*;
pub use time_period::*;
mod addressable;
mod characteristic;
mod characteristic_relationship;
mod entity_ref;
mod entity_ref_fvo;
mod entity_ref_mvo;
mod extensible;
mod hub;
mod party_ref_or_party_role_ref;
mod promotion;
mod promotion_action;
mod promotion_action_fvo;
mod promotion_action_mvo;
mod promotion_criteria;
mod promotion_criteria_group;
mod promotion_criteria_group_fvo;
mod promotion_criteria_group_mvo;
mod promotion_criteria_fvo;
mod promotion_criteria_mvo;
mod promotion_pattern;
mod promotion_pattern_fvo;
mod promotion_pattern_mvo;
mod related_party_ref_or_party_role_ref;
mod time_period;
