#![allow(unused_imports)]
use tmf_642::model::*;
use tmf_642::Tmf642Client;
use tmf_642::request::create_comment_alarm::CreateCommentAlarmRequired;
#[tokio::main]
async fn main() {
    let client = Tmf642Client::from_env();
    let alarm_pattern = vec![AlarmRefOrValue {}];
    let comment = Comment {
        extensible: Extensible {
            base_type: Some("your @base type".to_owned()),
            schema_location: Some("your @schema location".to_owned()),
            type_: "your @type".to_owned(),
        },
        comment: Some("your comment".to_owned()),
        system_id: Some("your system id".to_owned()),
        time: Some(chrono::Utc::now()),
        user_id: Some("your user id".to_owned()),
    };
    let commented_alarm = vec![AlarmRefOrValue {}];
    let href = "your href";
    let id = "your id";
    let state = "your state";
    let response = client
        .create_comment_alarm(CreateCommentAlarmRequired {
            alarm_pattern,
            comment,
            commented_alarm,
            href,
            id,
            state,
        })
        .fields("your fields")
        .await
        .unwrap();
    println!("{:#?}", response);
}
