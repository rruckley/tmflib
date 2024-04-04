//! Create Document Example
//! 
use tmflib::tmf667::document::Document;

fn main() {
    let doc = Document::new("My Document")
        .doc_type("PDF");

    dbg!(doc);
}