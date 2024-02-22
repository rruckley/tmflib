//! Sample Example Module

// Running this example forces an execution of build.rs if build.rs
// has changes or any of the input OAS JSON files have changed.
// We then include the resulting generated auto-lib.rs parent library
// to make use of the generated modules.

include!(concat!(env!("OUT_DIR"), "/auto-lib.rs"));

// This module and struct are auto-generated
//use tmf723::policy_rule::PolicyRule;

use openapiv3::OpenAPI;

fn main() {
    // let rule = PolicyRule {
    //     default_0: "A Field".to_string(),
    // };
    println!("There is a message about auto-generation");

    // Open a OAS file
    //let data = include_str!("../open_api/TMF723-Policy_Management-v5.0.0.oas.json");
    //let openapi : OpenAPI = serde_json::from_str(data).expect("Could not parse YAML");

    //dbg!(openapi);
}