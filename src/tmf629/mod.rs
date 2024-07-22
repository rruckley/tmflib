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

//! TMF629 Customer Management
//! # Description
//! This module represents the common case of a customer. Customer is a specific instance of a Party Role [crate::tmf669]. 
//! This means to create a customer, an organisation (from [`crate::tmf632`]) must be supplied. 

const MOD_PATH: &str = "customerManagement/v4";

pub mod characteristic;
pub mod customer;
