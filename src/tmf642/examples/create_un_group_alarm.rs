#![allow(unused_imports)]
use tmf_642::model::*;
use tmf_642::Tmf642Client;
use tmf_642::request::create_un_group_alarm::CreateUnGroupAlarmRequired;
#[tokio::main]
async fn main() {
    let client = Tmf642Client::from_env();
    let alarm_changed_time = chrono::Utc::now();
    let correlated_alarm = vec![AlarmRefOrValue {}];
    let href = "your href";
    let id = "your id";
    let parent_alarm = AlarmRefOrValue {};
    let source_system_id = "your source system id";
    let state = "your state";
    let un_grouped_alarm = vec![AlarmRefOrValue {}];
    let response = client
        .create_un_group_alarm(CreateUnGroupAlarmRequired {
            alarm_changed_time,
            correlated_alarm,
            href,
            id,
            parent_alarm,
            source_system_id,
            state,
            un_grouped_alarm,
        })
        .fields("your fields")
        .await
        .unwrap();
    println!("{:#?}", response);
}
