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

//! TMF676 Payment Management

#[cfg(feature = "build-V4")]
const MOD_PATH: &str = "paymentManagement/v4";
#[cfg(feature = "build-V5")]
const MOD_PATH: &str = "paymentManagement/v5";

// For payment method
const CLASS_PATH: &str = "method";

pub mod payment;
pub mod refund;

use crate::{HasDescription, HasId, HasName, Uri};
use serde::{Deserialize, Serialize};
use tmflib_derive::{HasDescription, HasId, HasName};

/// Payment Method
#[derive(Clone, Debug, Default, HasId, HasName, HasDescription, Serialize, Deserialize)]
pub struct PaymentMethodRefOrValue {
    id: Option<String>,
    href: Option<Uri>,
    name: Option<String>,
    description: Option<String>,
}
