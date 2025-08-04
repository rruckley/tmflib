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

//! TMF678 Customer Bill Management
//! # Description
//! Provides schema for management of [`customer_bill_v4::CustomerBill`]s.
//! # Versions
//! - V4 Supported
//! - V5 Supported

const MOD_PATH: &str = "tmf678/v5";

#[cfg(all(feature = "tmf678", feature = "build-V4"))]
pub mod customer_bill_v4;

#[cfg(all(feature = "tmf678", feature = "build-V5"))]
pub mod customer_bill_v5;
