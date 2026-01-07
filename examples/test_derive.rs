//! Test something
//!

#[derive(Default)]
struct Test {
    vec: Option<Vec<String>>,
}

fn main() {
    let mut test = Test::default();
    test.vec = Some(vec!["A String".to_string()]);

    let my_test = &test;
    let rp = match my_test.vec.as_ref() {
        Some(v) => match v.get(0) {
            Some(i) => Some(i).cloned(),
            None => None,
        },
        None => None,
    };
    dbg!(rp);
}
