//! Example using the client 
//! 
//! 

#[cfg(feature = "client")]
use tmflib::client::TMFClient;

fn main() {
    #[cfg(feature = "client")]
    let client = TMFClient::new("http://localhost:8080");

    // How do we wish to use this client? Ideally, to make it easier for a client program to interact with and consume
    // data from TMF APIs.
    // To that end we would want to do something like this:
    // catalogs : Vec<Catalog> = client.tmf620().catalog().list();
    // Whilst this is similar to what platypus does (provides the APIs), it the other end of it.
    // So we're asking for a client to consume from platypus or any other TMF compliant source.

    #[cfg(feature = "client")]
    dbg!(client);
}