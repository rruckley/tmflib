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

//! TMF666 Billing Account Management
//! # Description
//! Provides schema for management of billing accounts.
//! # Versions
//! - V4 Supported

use serde::{Deserialize, Serialize};

use crate::common::money::Money;
use crate::TimePeriod;

pub mod billing_account;
pub mod financial_account;
pub mod party_account;
pub mod settlement_account;

use crate::HasValidity;
use tmflib_derive::HasValidity;

const MOD_PATH: &str = "accountManagement/v4";

/// Account Reference
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRef {
    /// Referenced Account Description
    pub description: Option<String>,
    /// Referenced Account HREF
    pub href: String,
    /// Referenced Account Unique Id
    pub id: String,
    /// Referenced Account Name
    pub name: String,
}

/// Account Relationship
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRelationship {
    /// Account Relationship Type
    pub relationship_type: String,
    /// Account Relationship Validity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    /// Linked Account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountRef>,
}

/// Account Balance
#[derive(Clone, Debug, Default, Deserialize, HasValidity, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<Money>,
    balance_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
}

/// Tax Exemption Status
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTaxExemption {
    certificate_number: String,
    issuing_jurisdiction: String,
    reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
}

/// Reference to a Payment Plan
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentPlanRef {
    href: String,
    id: String,
    name: String,
}

/// a Payment Plan
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentPlan {
    number_of_payments: u32,
    payment_frequency: String,
    plan_type: String,
    priority: u16,
    status: String,
    total_amount: Option<Money>,
    valid_for: Option<TimePeriod>,
    // Referenced Data
    payment_method: Option<PaymentMethodRef>,
}

/// Reference to a Payment Method
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodRef {
    href: String,
    id: String,
    name: String,
}

#[cfg(test)]
mod test {
    use super::{AccountBalance, AccountRef, PaymentMethodRef, PaymentPlan};

    const ACCOUNTREF_JSON: &str = "{
        \"name\" : \"AccountName\",
        \"id\" : \"ACC123\",
        \"href\" : \"http://example.com/tmf666/account/ACC123\"        
    }";

    const ACCOUNTBAL_JSON: &str = "{
        \"balanceType\" : \"Remaining\"
    }";

    const PP_JSON: &str = "{
        \"numberOfPayments\" : 12,
        \"paymentFrequency\" : \"Monthly\",
        \"planType\" : \"12Month\",
        \"priority\" : 1,
        \"status\" : \"New\"
    }";

    const PMREF_JSON: &str = "{
        \"name\" : \"PaymentMethod\",
        \"id\" : \"PM123\",
        \"href\" : \"http://example.com/tmf666/method/PM123\"
    }";

    #[test]
    fn test_accountref_deserialize() {
        let accountref: AccountRef = serde_json::from_str(ACCOUNTREF_JSON).unwrap();

        assert_eq!(accountref.name.as_str(), "AccountName");
        assert_eq!(accountref.id.as_str(), "ACC123");
        assert_eq!(
            accountref.href.as_str(),
            "http://example.com/tmf666/account/ACC123"
        );
    }

    #[test]
    fn test_accountbal_deserialize() {
        let accountbal: AccountBalance = serde_json::from_str(ACCOUNTBAL_JSON).unwrap();

        assert_eq!(accountbal.balance_type.as_str(), "Remaining");
    }

    #[test]
    fn test_pp_deserialize() {
        let paymentplan: PaymentPlan = serde_json::from_str(PP_JSON).unwrap();

        assert_eq!(paymentplan.number_of_payments, 12);
        assert_eq!(paymentplan.payment_frequency.as_str(), "Monthly");
        assert_eq!(paymentplan.plan_type.as_str(), "12Month");
        assert_eq!(paymentplan.priority, 1);
        assert_eq!(paymentplan.status.as_str(), "New");
    }

    #[test]
    fn test_pmref_deserialize() {
        let pmref: PaymentMethodRef = serde_json::from_str(PMREF_JSON).unwrap();

        assert_eq!(
            pmref.href.as_str(),
            "http://example.com/tmf666/method/PM123"
        );
        assert_eq!(pmref.id.as_str(), "PM123");
        assert_eq!(pmref.name.as_str(), "PaymentMethod");
    }
}
