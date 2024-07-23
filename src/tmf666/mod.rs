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

//! TMF666 Billing Account Management
//! # Description
//! Provides schema for management of billing accounts.
//! # Versions
//! - V4 Supported

use serde::{Deserialize, Serialize};

use crate::TimePeriod;
use crate::common::money::Money;

pub mod billing_account;
pub mod party_account;
pub mod financial_account;
pub mod settlement_account;

const MOD_PATH : &str = "accountManagement/v4";

/// Account Reference
#[derive( Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRef {
    description: Option<String>,
    href: String,
    id: String,
    name : String,
}

/// Account Relationship
#[derive( Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRelationship {
    relationship_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<AccountRef>,
}

/// Account Balance
#[derive( Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount : Option<Money>,
    balance_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
}

/// Tax Exemption Status
#[derive( Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTaxExemption {
    certificate_number: String,
    issuing_jurisdiction: String,
    reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
}