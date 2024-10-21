//! Create Document Example
//! 
use tmflib::tmf667::document::Document;
use tmflib::tmf651::agreement::Agreement;

fn main() {
    let agreement = Agreement::new("My Aggreement");
    
    let doc = Document::new("My Document")
        .doc_type("PDF")
        .link(agreement);


    // doc.link_entity(agreement);

    dbg!(doc);
}