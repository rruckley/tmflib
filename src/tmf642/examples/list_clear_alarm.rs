#![allow(unused_imports)]
use tmf_642::model::*;
use tmf_642::Tmf642Client;
#[tokio::main]
async fn main() {
    let client = Tmf642Client::from_env();
    let response = client
        .list_clear_alarm()
        .fields("your fields")
        .limit(1)
        .offset(1)
        .await
        .unwrap();
    println!("{:#?}", response);
}
