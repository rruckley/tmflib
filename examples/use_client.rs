//! Example using the client 
//! 
//! 

#[cfg(feature = "client")]
use tmflib::client::TMFClient;

fn main() {
    #[cfg(feature = "client")]
    let client = TMFClient::new("http://localhost:8080");

    #[cfg(feature = "client")]
    dbg!(client);
}