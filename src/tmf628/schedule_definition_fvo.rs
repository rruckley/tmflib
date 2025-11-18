use serde::{Serialize, Deserialize};
use super::{
    DayOfWeekRecurrenceFvo, ExtensibleFvo, MonthlyScheduleDayOfWeekDefinitionFvo,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScheduleDefinitionFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    #[serde(rename = "MonthlyScheduleDayOfWeekDefinition")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monthly_schedule_day_of_week_definition: Option<
        MonthlyScheduleDayOfWeekDefinitionFvo,
    >,
    ///The weekly schedule is used to define a schedule that is based on the days of the week, e.g. a schedule that will be active only on Monday and Tuesday.
    #[serde(rename = "WeeklyScheduledDefinition")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weekly_scheduled_definition: Vec<DayOfWeekRecurrenceFvo>,
    ///The date schedule is used to define a schedule that is based on specific dates, such as December 31st 2015, February 28th 2013
    #[serde(rename = "dateScheduleDefintion")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub date_schedule_defintion: Vec<crate::DateTime>,
    ///A list of specific dates that should be excluded from the Schedule Definition.
    #[serde(rename = "excludedDate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_date: Vec<chrono::DateTime<chrono::Utc>>,
    ///The schedule definition for running the threshold job
    #[serde(rename = "monthlyScheduleDayOfMonthDefinition")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monthly_schedule_day_of_month_definition: Vec<chrono::DateTime<chrono::Utc>>,
    ///A recurring frequency to run a job within day that is included in schedule definition, for example: every 5 minutes, 15 minute, 30 minutes, 1 hour
    #[serde(rename = "recurringFrequency")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring_frequency: Option<String>,
    ///The End time of the Schedule Definition. If the attribute is empty the Schedule run forever, not having a time constraint.
    #[serde(rename = "scheduleDefinitionEndTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_definition_end_time: Option<chrono::DateTime<chrono::Utc>>,
    ///A list of time ranges within a specific day that the schedule will be active on, for example 08:00-12:00, 16:00-19:00.
    #[serde(rename = "scheduleDefinitionHourRange")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_definition_hour_range: Option<String>,
    ///The Start time of the Schedule Definition
    #[serde(rename = "scheduleDefinitionStartTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_definition_start_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for ScheduleDefinitionFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ScheduleDefinitionFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for ScheduleDefinitionFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
