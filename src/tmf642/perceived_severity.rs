//! Defines the PerceivedSeverity enum used in Alarm definitions.

use serde::{Deserialize, Serialize};
/**Lists the possible severities that can be allocated to an Alarm. The values are consistent with ITU-T Recommendation X.733.
Once an alarm has been cleared, its perceived severity is set to 'cleared' and can no longer be set.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PerceivedSeverity {
    /// Critical Severity
    #[serde(rename = "critical")]
    Critical,
    /// Major Severity
    #[serde(rename = "major")]
    Major,
    /// Minor Severity
    #[serde(rename = "minor")]
    Minor,
    /// Warning Severity
    #[serde(rename = "warning")]
    Warning,
    /// Indeterminate Severity
    #[serde(rename = "indeterminate")]
    Indeterminate,
    /// Cleared Severity
    #[serde(rename = "cleared")]
    Cleared,
}
