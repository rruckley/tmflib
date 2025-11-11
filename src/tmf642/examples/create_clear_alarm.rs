#![allow(unused_imports)]
use tmf_642::model::*;
use tmf_642::Tmf642Client;
use tmf_642::request::create_clear_alarm::CreateClearAlarmRequired;
#[tokio::main]
async fn main() {
    let client = Tmf642Client::from_env();
    let alarm_cleared_time = chrono::Utc::now();
    let alarm_pattern = vec![AlarmRefOrValue {}];
    let clear_system_id = "your clear system id";
    let clear_user_id = "your clear user id";
    let cleared_alarm = vec![AlarmRefOrValue {}];
    let state = "your state";
    let response = client
        .create_clear_alarm(CreateClearAlarmRequired {
            alarm_cleared_time,
            alarm_pattern,
            clear_system_id,
            clear_user_id,
            cleared_alarm,
            state,
        })
        .fields("your fields")
        .await
        .unwrap();
    println!("{:#?}", response);
}
