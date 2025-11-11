#![allow(unused_imports)]
use tmf_642::model::*;
use tmf_642::Tmf642Client;
use tmf_642::request::create_un_ack_alarm::CreateUnAckAlarmRequired;
#[tokio::main]
async fn main() {
    let client = Tmf642Client::from_env();
    let ack_system_id = "your ack system id";
    let ack_time = chrono::Utc::now();
    let ack_user_id = "your ack user id";
    let alarm_pattern = vec![AlarmRefOrValue {}];
    let href = "your href";
    let id = "your id";
    let state = "your state";
    let un_acked_alarm = vec![AlarmRefOrValue {}];
    let response = client
        .create_un_ack_alarm(CreateUnAckAlarmRequired {
            ack_system_id,
            ack_time,
            ack_user_id,
            alarm_pattern,
            href,
            id,
            state,
            un_acked_alarm,
        })
        .fields("your fields")
        .await
        .unwrap();
    println!("{:#?}", response);
}
