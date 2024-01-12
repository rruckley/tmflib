//! Module to test HasId derive macro


use tmflib_derive::HasId;
use tmflib::HasId;

// Defined required consts
const CLASS_PATH : &str = "mytmf";
const LIB_PATH : &str = "tmf-lib";
const MOD_PATH : &str = "tmf7xx";

#[derive(HasId)]
struct MyTMF {
    pub id : String,
    pub href : String,
}

fn main() {
    let mut my_tmf = MyTMF {
        id : "123".to_string(),
        href: "http://me".to_string(),
    };

    my_tmf.generate_href();
    let href = my_tmf.get_href();

    dbg!(href);
}