use serde::{Serialize, Deserialize};
/**Lists the possible severities that can be allocated to an Alarm. The values are consistent with ITU-T Recommendation X.733.
Once an alarm has been cleared, its perceived severity is set to 'cleared' and can no longer be set.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PerceivedSeverity {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "major")]
    Major,
    #[serde(rename = "minor")]
    Minor,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "indeterminate")]
    Indeterminate,
    #[serde(rename = "cleared")]
    Cleared,
}
