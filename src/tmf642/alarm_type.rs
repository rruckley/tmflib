use serde::{Serialize, Deserialize};
///Categorizes the alarm (X.733 8.1.1, 3GPP TS 32.111-2 Annex A)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AlarmType {
    #[serde(rename = "communicationsAlarm")]
    CommunicationsAlarm,
    #[serde(rename = "processingErrorAlarm")]
    ProcessingErrorAlarm,
    #[serde(rename = "environmentalAlarm")]
    EnvironmentalAlarm,
    #[serde(rename = "qualityOfServiceAlarm")]
    QualityOfServiceAlarm,
    #[serde(rename = "equipmentAlarm")]
    EquipmentAlarm,
    #[serde(rename = "integrityViolation")]
    IntegrityViolation,
    #[serde(rename = "operationalViolation")]
    OperationalViolation,
    #[serde(rename = "physicalViolation")]
    PhysicalViolation,
    #[serde(rename = "securityService")]
    SecurityService,
    #[serde(rename = "mechanismViolation")]
    MechanismViolation,
    #[serde(rename = "timeDomainViolation")]
    TimeDomainViolation,
}
