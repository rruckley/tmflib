//! Module to test HasId derive macro

use tmflib::{get_lib_path, HasId};
use tmflib_derive::HasId;

// URL Path components
// use super::TMF_MODULE;
const TMF_MODULE: &str = "testModule";
use tmflib::TMF_VERSION;
// Defined required consts
const CLASS_PATH: &str = "mytmf";

#[derive(Default, HasId)]
struct MyTMF {
    pub id: Option<String>,
    pub href: Option<String>,
}

fn main() {
    let mut my_tmf = MyTMF {
        id: Some("123".to_string()),
        href: Some("http://me".to_string()),
    };

    my_tmf.generate_href();
    let href = my_tmf.get_href();

    dbg!(href);
}
