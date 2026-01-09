//! Sample Example Module

// Running this example forces an execution of build.rs if build.rs
// has changes or any of the input OAS JSON files have changed.
// We then include the resulting generated auto-lib.rs parent library
// to make use of the generated modules.

// include!(concat!(env!("OUT_DIR"), "/auto-lib.rs"));

// This module and struct are auto-generated
use tmf723::managed_policy::ManagedPolicy;

fn main() {
    let policy = ManagedPolicy::default();
    dbg!(policy);
    println!("There is a message about auto-generation");
}