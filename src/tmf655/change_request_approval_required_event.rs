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

use crate::DateTime;

use super::ChangeRequestApprovalRequiredEventPayload;
use serde::{Deserialize, Serialize};
///The notification data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeRequestApprovalRequiredEvent {
    ///The correlation id for this event.
    #[serde(rename = "correlationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    ///An explnatory of the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The domain of the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    ///The event data structure
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<ChangeRequestApprovalRequiredEventPayload>,
    ///The identifier of the notification.
    #[serde(rename = "eventId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    ///Time of the event occurrence.
    #[serde(rename = "eventTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<DateTime>,
    ///The type of the notification.
    #[serde(rename = "eventType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    ///A priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    ///The time the event occured.
    #[serde(rename = "timeOcurred")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_ocurred: Option<DateTime>,
    ///The title of the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl std::fmt::Display for ChangeRequestApprovalRequiredEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
