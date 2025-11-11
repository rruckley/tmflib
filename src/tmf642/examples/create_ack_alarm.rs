#![allow(unused_imports)]
use tmf_642::model::*;
use tmf_642::Tmf642Client;
use tmf_642::request::create_ack_alarm::CreateAckAlarmRequired;
#[tokio::main]
async fn main() {
    let client = Tmf642Client::from_env();
    let ack_system_id = "your ack system id";
    let ack_time = chrono::Utc::now();
    let ack_user_id = "your ack user id";
    let acked_alarm = vec![AlarmRefOrValue {}];
    let alarm_pattern = vec![AlarmRefOrValue {}];
    let state = "your state";
    let response = client
        .create_ack_alarm(CreateAckAlarmRequired {
            ack_system_id,
            ack_time,
            ack_user_id,
            acked_alarm,
            alarm_pattern,
            state,
        })
        .fields("your fields")
        .await
        .unwrap();
    println!("{:#?}", response);
}
