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
//! Alarm defines an alarm for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type

use super::{
    AlarmRef, AlarmType, AlarmedObjectRef, Comment, CrossedThresholdInformation, PerceivedSeverity,
    RelatedPlace, ServiceRef, MOD_PATH,
};
use crate::{common::entity::Entity, DateTime, HasId, Uri};
use serde::{Deserialize, Serialize};

const CLASS_PATH: &str = "alarm";

use tmflib_derive::HasId;

///Alarm defines an alarm for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
#[derive(Default, Debug, Clone, Serialize, Deserialize, HasId)]
pub struct Alarm {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    /// Unique identifier of the alarm
    pub id: Option<String>,
    /// Hyperlink to the alarm
    pub href: Option<Uri>,
    ///Provides the Acknowledgement State of the alarm (unacknowledged, acknowledged).
    #[serde(rename = "ackState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ack_state: Option<String>,
    ///Provides the name of the system that last changed the ackState of an alarm, i.e. acknowledged or unacknowledged the alarm.
    #[serde(rename = "ackSystemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ack_system_id: Option<String>,
    ///Provides the id of the user who has last changed the ack state of the alarm, i.e. acknowledged or unacknowledged the alarm.
    #[serde(rename = "ackUserId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ack_user_id: Option<String>,
    ///Provides list of affected services.
    #[serde(rename = "affectedService")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub affected_service: Vec<ServiceRef>,
    ///Indicates the last date and time when the alarm is changed on the alarm-owning system. Any change to the alarm whether coming from the alarmed resource, or triggered by a change from the client is changing this time.
    #[serde(rename = "alarmChangedTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarm_changed_time: Option<DateTime>,
    ///Indicates the time (as a date + time) at which the alarm is cleared at the source.
    #[serde(rename = "alarmClearedTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarm_cleared_time: Option<DateTime>,
    ///Contains further information on the alarm.
    #[serde(rename = "alarmDetails")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarm_details: Option<String>,
    ///Indicates if this alarm has been escalated or not.
    #[serde(rename = "alarmEscalation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarm_escalation: Option<bool>,
    ///Indicates the time (as a date + time) at which the alarm occurred at its source.
    #[serde(rename = "alarmRaisedTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarm_raised_time: Option<DateTime>,
    /**Indicates the time (as a date + time) at which the alarm was reported by the owning OSS. It might be different from the alarmRaisedTime. For instance, if the alarm list is maintained by an EMS, the alarmRaisedtime would be the time the alarm
    was detected by the NE, while the alarmReportingTime would be the time this alarm was stored in the alarm list of the EMS.*/
    #[serde(rename = "alarmReportingTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarm_reporting_time: Option<DateTime>,
    ///Categorizes the alarm (X.733 8.1.1, 3GPP TS 32.111-2 Annex A)
    #[serde(rename = "alarmType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarm_type: Option<AlarmType>,
    ///Reference to object which affected by Alarm (AlarmedObject).
    #[serde(rename = "alarmedObject")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarmed_object: Option<AlarmedObjectRef>,
    ///The type (class) of the managed object associated with the event.
    #[serde(rename = "alarmedObjectType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarmed_object_type: Option<String>,
    ///Provides the id of the system where the user who invoked the alarmCleared operation is located.
    #[serde(rename = "clearSystemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clear_system_id: Option<String>,
    ///Provides the id of the user who invoked the alarmCleared operation
    #[serde(rename = "clearUserId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clear_user_id: Option<String>,
    ///Provides list of Comments regards Alarm.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<Comment>,
    ///Provides list of correlated Alarms.
    #[serde(rename = "correlatedAlarm")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub correlated_alarm: Vec<AlarmRef>,
    ///Identifies the details of the threshold that has been crossed.
    #[serde(rename = "crossedThresholdInformation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crossed_threshold_information: Option<CrossedThresholdInformation>,
    ///An identifier of the alarm in the source system.
    #[serde(rename = "externalAlarmId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_alarm_id: Option<String>,
    ///Indicates whether the alarm is a root cause alarm..
    #[serde(rename = "isRootCause")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_root_cause: Option<bool>,
    ///Provides list of parrent Alarms.
    #[serde(rename = "parentAlarm")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent_alarm: Vec<AlarmRef>,
    /**Lists the possible severities that can be allocated to an Alarm. The values are consistent with ITU-T Recommendation X.733.
    Once an alarm has been cleared, its perceived severity is set to 'cleared' and can no longer be set.*/
    #[serde(rename = "perceivedSeverity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perceived_severity: Option<PerceivedSeverity>,
    ///List of related places, which are affected by Alarm.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub place: Vec<RelatedPlace>,
    ///Indicates that the Managed Object (related to this alarm) is in planned outage (in planned maintenance, or out-of-service).
    #[serde(rename = "plannedOutageIndicator")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planned_outage_indicator: Option<String>,
    ///Provides the probable cause of the alarm. The values are consistent with ITU-T Recommendation X.733 or 3GPP TS 32.111-2 Annex B.
    #[serde(rename = "probableCause")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probable_cause: Option<String>,
    ///Indicates proposed repair actions, if known to the system emitting the alarm.
    #[serde(rename = "proposedRepairedActions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proposed_repaired_actions: Option<String>,
    ///Reporting system identity.
    #[serde(rename = "reportingSystemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reporting_system_id: Option<String>,
    ///Indicates whether the alarm affects service or not.
    #[serde(rename = "serviceAffecting")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_affecting: Option<bool>,
    ///Source system identity.
    #[serde(rename = "sourceSystemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,
    ///Provides more specific information about the alarm.
    #[serde(rename = "specificProblem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specific_problem: Option<String>,
    ///Defines the alarm state during its life cycle (raised, updated, cleared).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl std::fmt::Display for Alarm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Alarm {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Alarm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
