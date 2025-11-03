use std::collections::HashMap;

pub fn hashmap_example() {
    let mut col = HashMap::new();
    col.insert("pera", "detlic");
    println!("{:?}", &col);
    if let Some(value) = col.get_mut("pera") {
        *value = "vise nije detlic";
    }
    println!("{:?}", &col);
}

fn main() {
    println!("=========================");
    hashmap_example();
}
