//! Validate customer code generation
//! 
use tmflib::{tmf629::customer::Customer, HasName,HasId};

fn main() {

    let mut cust1 = Customer::default();

    cust1.set_name("NATIONAL BEVERAGE COMPANY LIMITED");
    cust1.id = Some(String::from("123456"));
    cust1.generate_code(None);

    let code1 = cust1.get_characteristic("code");
    let hash = cust1.get_characteristic("hash");
    let sha = cust1.get_characteristic("sha");

    println!("Customer: {} + UID: {}\t generates Code: {}",cust1.get_name(),cust1.get_id(),code1.unwrap().value);
    println!("Customer: {},\tSHA: {}",cust1.get_name(),sha.unwrap().value);
    println!("Customer: {},\tBase32: {}",cust1.get_name(),hash.unwrap().value);

    cust1.generate_code(Some(1));

    let code1 = cust1.get_characteristic("code");
    let hash = cust1.get_characteristic("hash");
    let sha = cust1.get_characteristic("sha");

    println!("Customer: {} + UID: {}\t generates Code: {}",cust1.get_name(),cust1.get_id(),code1.unwrap().value);
    println!("Customer: {},\tSHA: {}",cust1.get_name(),sha.unwrap().value);
    println!("Customer: {},\tBase32: {}",cust1.get_name(),hash.unwrap().value);
}