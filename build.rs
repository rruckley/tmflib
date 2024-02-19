//! Build module to generate new modules from OAS / Swagger files
//! 

extern crate openapi;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("sample.rs");
    fs::write(
        &dest_path,
        "
        pub fn message() -> &'static str {
            \"Geeks for TMF!\"
        }
        "
    ).unwrap();
    // Generate output for each Swagger / OAS file found

    println!("cargo:rerun-if-changed=build.rs");
}