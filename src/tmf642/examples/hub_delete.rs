#![allow(unused_imports)]
use tmf_642::model::*;
use tmf_642::Tmf642Client;
#[tokio::main]
async fn main() {
    let client = Tmf642Client::from_env();
    let id = "your id";
    let response = client.hub_delete(id).await.unwrap();
    println!("{:#?}", response);
}
