//! Validate customer code generation
//!
use tmflib::{tmf629::customer::Customer, HasId, HasName};

fn main() -> Result<(), String> {
    let mut cust1 = Customer::default();

    cust1.set_name("NATIONAL BEVERAGE COMPANY LIMITED");
    cust1.id = Some(String::from("123456"));
    cust1.generate_code(None);

    let code1 = cust1
        .get_characteristic("code")
        .ok_or(String::from("No Value"))?;
    let hash = cust1
        .get_characteristic("hash")
        .ok_or(String::from("No Value"))?;

    println!(
        "Customer: {} + ID: {} Offset=0\t generates Code: {}",
        cust1.get_name(),
        cust1.get_id(),
        code1.value
    );
    println!("Customer: {},\tBase32: {}", cust1.get_name(), hash.value);

    cust1.generate_code(Some(1));

    let code1 = cust1
        .get_characteristic("code")
        .ok_or(String::from("No Value"))?;
    let hash = cust1
        .get_characteristic("hash")
        .ok_or(String::from("No Value"))?;

    println!(
        "Customer: {} + ID: {} Offset=1\t generates Code: {}",
        cust1.get_name(),
        cust1.get_id(),
        code1.value
    );
    println!("Customer: {},\tBase32: {}", cust1.get_name(), hash.value);

    Ok(())
}
