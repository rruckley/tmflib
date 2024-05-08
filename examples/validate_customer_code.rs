//! Validate customer code generation
//! 
use tmflib::{tmf629::customer::Customer, HasName,HasId};

fn main() {

    let mut cust1 = Customer::default();

    cust1.set_name("AUSTRALIA AND NEW ZEALAND BANKING GROUP LIMITED");
    cust1.id = Some(String::from("4317"));
    cust1.generate_code(None);

    let code1 = cust1.get_characteristic("code");
    let hash = cust1.get_characteristic("hash");
    let sha = cust1.get_characteristic("sha");

    println!("Customer: {} + UID: {}\t generates Code: {}",cust1.get_name(),cust1.get_id(),code1.unwrap().value);
    println!("Customer: {},\tSHA: {}",cust1.get_name(),sha.unwrap().value);
    println!("Customer: {},\tBase32: {}",cust1.get_name(),hash.unwrap().value);


    let mut cust2 = Customer::default();

    cust2.set_name("CAPITAL FINANCE AUSTRALIA");
    cust2.id = Some(String::from("6132"));
    cust2.generate_code(None);

    let code = cust2.get_characteristic("code");
    let hash = cust2.get_characteristic("hash");
    let sha = cust2.get_characteristic("sha");

    println!("Customer: {},\tUID: {}\t generates Code: {}",cust2.get_name(),cust2.get_id(),code.unwrap().value);
    println!("Customer: {},\tSHA: {}",cust2.get_name(),sha.unwrap().value);
    println!("Customer: {},\tBase32: {}",cust2.get_name(),hash.unwrap().value);
}