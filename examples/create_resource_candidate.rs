//! Create Resource Candidate Example


use tmflib::tmf634::resource_candidate::ResourceCandidate;

fn main() {
    let rc = ResourceCandidate::new("My RC".into())
        .description("This is a description");

    dbg!(rc);
}