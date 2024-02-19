//! Build module to generate new modules from OAS / Swagger files
//! 

extern crate openapi;

fn main() {
    // Generate output for each Swagger / OAS file found
    match openapi::from_path("open_api//home/rruckley/build/tmflib/open_api/TMF723-Policy_Management-v5.0.0.oas.yaml") {
        Ok(spec)     => println!("spec: {:?}", spec),
        Err(err) => println!("error: {}",err),
    }
}