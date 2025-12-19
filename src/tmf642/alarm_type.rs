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
//! Categorizes the alarm

use serde::{Deserialize, Serialize};
///Categorizes the alarm (X.733 8.1.1, 3GPP TS 32.111-2 Annex A)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AlarmType {
    ///No alarm type specified
    #[serde(rename = "communicationsAlarm")]
    CommunicationsAlarm,
    ///Indicates a device or system alarm
    #[serde(rename = "processingErrorAlarm")]
    /// Indicates a device or system alarm
    ProcessingErrorAlarm,
    /// Indicates a device or system alarm
    #[serde(rename = "environmentalAlarm")]
    EnvironmentalAlarm,
    /// Quality Of Service Alarm
    #[serde(rename = "qualityOfServiceAlarm")]
    QualityOfServiceAlarm,
    /// Equipment Alarm
    #[serde(rename = "equipmentAlarm")]
    EquipmentAlarm,
    /// Integrity Violation
    #[serde(rename = "integrityViolation")]
    IntegrityViolation,
    /// Operational Violation
    #[serde(rename = "operationalViolation")]
    OperationalViolation,
    /// Physical Violation
    #[serde(rename = "physicalViolation")]
    PhysicalViolation,
    /// Security Service
    #[serde(rename = "securityService")]
    SecurityService,
    /// Mechanism Violation
    #[serde(rename = "mechanismViolation")]
    MechanismViolation,
    /// Time Domain Violation
    #[serde(rename = "timeDomainViolation")]
    TimeDomainViolation,
}
