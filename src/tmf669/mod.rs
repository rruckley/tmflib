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

//! TMF669 Party Role Management
//! # Description
//! This API can be seen as a generalisation of CUstomer management API where Party Roles may be any, not only a customer. 
//! In fact, Customer  ([tmf629]) can be seen as a specific instance of a party role. 
//! The role defines a specialisation of how the party object is used and thus a reference to a party (via [tmf632]) is required.
//! This is achieved via the [common/related_party] object.

const MOD_PATH : &str = "partyRoleManagement/v4";

pub mod party_role;