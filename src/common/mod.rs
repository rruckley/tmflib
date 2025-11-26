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

//! Common Modules

const MOD_PATH: &str = "common";

pub mod addressable;
pub mod attachment;
pub mod contact;
pub mod entity;
pub mod event;
pub mod external_identifier;
pub mod extensible;
pub mod money;
pub mod note;
pub mod price;
pub mod product;
pub mod related_entity;
pub mod related_party;
pub mod related_place;
pub mod tax_item;

pub mod tmf_error;
