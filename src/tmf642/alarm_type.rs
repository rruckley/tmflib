use serde::{Serialize, Deserialize};
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
