#![allow(unused_imports)]
use tmf_642::model::*;
use tmf_642::Tmf642Client;
use tmf_642::request::un_group_alarm_create_event::UnGroupAlarmCreateEventRequired;
#[tokio::main]
async fn main() {
    let client = Tmf642Client::from_env();
    let base_type = "your @base type";
    let schema_location = "your @schema location";
    let type_ = "your @type";
    let analytic_characteristic = vec![
        Characteristic { extensible : Extensible { base_type : Some("your @base type"
        .to_owned()), schema_location : Some("your @schema location".to_owned()), type_ :
        "your @type".to_owned() }, characteristic_relationship :
        vec![CharacteristicRelationship { extensible : Extensible { base_type :
        Some("your @base type".to_owned()), schema_location :
        Some("your @schema location".to_owned()), type_ : "your @type".to_owned() }, id :
        Some("your id".to_owned()), relationship_type : Some("your relationship type"
        .to_owned()) }], id : Some("your id".to_owned()), name : Some("your name"
        .to_owned()), value_type : Some("your value type".to_owned()) }
    ];
    let correlation_id = "your correlation id";
    let description = "your description";
    let domain = "your domain";
    let event = serde_json::json!({});
    let event_id = "your event id";
    let event_time = chrono::Utc::now();
    let event_type = "your event type";
    let href = "your href";
    let id = "your id";
    let priority = "your priority";
    let related_party = vec![
        RelatedPartyRefOrPartyRoleRef { extensible : Extensible { base_type :
        Some("your @base type".to_owned()), schema_location :
        Some("your @schema location".to_owned()), type_ : "your @type".to_owned() },
        party_or_party_role : Some(PartyRefOrPartyRoleRef {}), role : Some("your role"
        .to_owned()) }
    ];
    let reporting_system = EntityRef {
        referred_type: Some("your @referred type".to_owned()),
        addressable: Addressable {
            href: Some("your href".to_owned()),
            id: Some("your id".to_owned()),
        },
        extensible: Extensible {
            base_type: Some("your @base type".to_owned()),
            schema_location: Some("your @schema location".to_owned()),
            type_: "your @type".to_owned(),
        },
        href: Some("your href".to_owned()),
        id: "your id".to_owned(),
        name: Some("your name".to_owned()),
    };
    let source = EntityRef {
        referred_type: Some("your @referred type".to_owned()),
        addressable: Addressable {
            href: Some("your href".to_owned()),
            id: Some("your id".to_owned()),
        },
        extensible: Extensible {
            base_type: Some("your @base type".to_owned()),
            schema_location: Some("your @schema location".to_owned()),
            type_: "your @type".to_owned(),
        },
        href: Some("your href".to_owned()),
        id: "your id".to_owned(),
        name: Some("your name".to_owned()),
    };
    let time_occurred = chrono::Utc::now();
    let title = "your title";
    let response = client
        .un_group_alarm_create_event(UnGroupAlarmCreateEventRequired {
            base_type,
            schema_location,
            type_,
            analytic_characteristic,
            correlation_id,
            description,
            domain,
            event,
            event_id,
            event_time,
            event_type,
            href,
            id,
            priority,
            related_party,
            reporting_system,
            source,
            time_occurred,
            title,
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}
