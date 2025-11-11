#![allow(unused_imports)]
use tmf_642::model::*;
use tmf_642::Tmf642Client;
use tmf_642::request::create_alarm::CreateAlarmRequired;
#[tokio::main]
async fn main() {
    let client = Tmf642Client::from_env();
    let ack_state = "your ack state";
    let ack_system_id = "your ack system id";
    let ack_user_id = "your ack user id";
    let affected_service = vec![ServiceRef {}];
    let alarm_changed_time = chrono::Utc::now();
    let alarm_cleared_time = chrono::Utc::now();
    let alarm_details = "your alarm details";
    let alarm_escalation = true;
    let alarm_raised_time = chrono::Utc::now();
    let alarm_reporting_time = chrono::Utc::now();
    let alarm_type = AlarmType::CommunicationsAlarm;
    let alarmed_object = AlarmedObjectRef {};
    let alarmed_object_type = "your alarmed object type";
    let clear_system_id = "your clear system id";
    let clear_user_id = "your clear user id";
    let comment = vec![
        Comment { extensible : Extensible { base_type : Some("your @base type"
        .to_owned()), schema_location : Some("your @schema location".to_owned()), type_ :
        "your @type".to_owned() }, comment : Some("your comment".to_owned()), system_id :
        Some("your system id".to_owned()), time : Some(chrono::Utc::now()), user_id :
        Some("your user id".to_owned()) }
    ];
    let correlated_alarm = vec![AlarmRef {}];
    let crossed_threshold_information = CrossedThresholdInformation {
        direction: Some("your direction".to_owned()),
        granularity: Some("your granularity".to_owned()),
        indicator_name: Some("your indicator name".to_owned()),
        indicator_unit: Some("your indicator unit".to_owned()),
        observed_value: Some("your observed value".to_owned()),
        threshold: Some(ThresholdRef {}),
        threshold_crossing_description: Some(
            "your threshold crossing description".to_owned(),
        ),
    };
    let external_alarm_id = "your external alarm id";
    let is_root_cause = true;
    let parent_alarm = vec![AlarmRef {}];
    let perceived_severity = PerceivedSeverity::Critical;
    let place = vec![
        RelatedPlaceFvo { extensible : Extensible { base_type : Some("your @base type"
        .to_owned()), schema_location : Some("your @schema location".to_owned()), type_ :
        "your @type".to_owned() }, related_place : PlaceFvo { entity : Entity {},
        external_identifier : vec![ExternalIdentifierFvo { extensible : Extensible {
        base_type : Some("your @base type".to_owned()), schema_location :
        Some("your @schema location".to_owned()), type_ : "your @type".to_owned() },
        external_identifier_type : Some("your external identifier type".to_owned()), id :
        "your id".to_owned(), owner : Some("your owner".to_owned()) }] }, role :
        "your role".to_owned() }
    ];
    let planned_outage_indicator = "your planned outage indicator";
    let probable_cause = "your probable cause";
    let proposed_repaired_actions = "your proposed repaired actions";
    let reporting_system_id = "your reporting system id";
    let service_affecting = true;
    let source_system_id = "your source system id";
    let specific_problem = "your specific problem";
    let state = "your state";
    let response = client
        .create_alarm(CreateAlarmRequired {
            ack_state,
            ack_system_id,
            ack_user_id,
            affected_service,
            alarm_changed_time,
            alarm_cleared_time,
            alarm_details,
            alarm_escalation,
            alarm_raised_time,
            alarm_reporting_time,
            alarm_type,
            alarmed_object,
            alarmed_object_type,
            clear_system_id,
            clear_user_id,
            comment,
            correlated_alarm,
            crossed_threshold_information,
            external_alarm_id,
            is_root_cause,
            parent_alarm,
            perceived_severity,
            place,
            planned_outage_indicator,
            probable_cause,
            proposed_repaired_actions,
            reporting_system_id,
            service_affecting,
            source_system_id,
            specific_problem,
            state,
        })
        .fields("your fields")
        .await
        .unwrap();
    println!("{:#?}", response);
}
