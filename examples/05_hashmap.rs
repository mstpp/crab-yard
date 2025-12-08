use std::collections::HashMap;

pub fn update_value_example() {
    let mut col: HashMap<&str, &str> = HashMap::new();
    println!("HashMap: {:?}", &col);

    col.insert("pera", "detlic");
    println!("HashMap: {:?}", &col);

    let init_op = col.insert("zdera", "metlic");
    println!("Option after first insert: {:?}", init_op);
    println!("HashMap: {:?}", &col);

    let dupl_op = col.insert("zdera", "----");
    println!("Option after duplicate insert: {:?}", dupl_op);
    println!("HashMap: {:?}", &col);

    // if let Some(value) = col.get_mut("pera") {
    //     *value = "PERICICH";
    // }
    col.entry("pera").and_modify(|v| *v = "PERICICH");

    println!("HashMap: {:?}", &col);
}

fn main() {
    update_value_example();
}
