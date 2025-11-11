#![allow(unused_imports)]
use tmf_642::model::*;
use tmf_642::Tmf642Client;
#[tokio::main]
async fn main() {
    let client = Tmf642Client::from_env();
    let body = serde_json::json!({});
    let response = client.create_hub(body).await.unwrap();
    println!("{:#?}", response);
}
