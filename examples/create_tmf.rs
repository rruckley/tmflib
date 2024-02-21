//! Sample Example Module

// Running this example forces an execution of build.rs if build.rs
// has changes or any of the input OAS JSON files have changed.
// We then include the resulting generated auto-lib.rs parent library
// to make use of the generated modules.

include!(concat!(env!("OUT_DIR"), "/auto-lib.rs"));

use tmf723::policy_rule::PolicyRule;

fn main() {
    let rule = PolicyRule {
        default_0: "A Field".to_string(),
    };
    println!("There is a message about auto-generation");

    dbg!(rule);
}