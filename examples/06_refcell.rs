use std::any::type_name;
use std::cell::RefCell;

fn print_type_of<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

struct PersonRefCell {
    name: RefCell<String>,
}

#[derive(Debug)]
struct PersonString {
    name: String,
}

pub fn refcell_example() {
    println!("p1 is immutable! but the name field is mutable");
    let p1 = PersonRefCell {
        name: RefCell::new(String::from("Joe")),
    };
    println!("p1 name: {:?}", p1.name);
    p1.name.replace(String::from("Jim"));
    println!("p1 new name: {:?}", p1.name);
    print_type_of(&p1.name);
    println!();

    let p2 = PersonString {
        name: String::from("Joe"),
    };
    println!("p2 name: {:?}", p2);
    print_type_of(&p2.name);
    println!();

    let c = RefCell::new(5);
    print_type_of(&c);

    let borrowed_five = c.borrow();
    let borrowed_five2 = c.borrow();
    println!("borrowed five {:?}", borrowed_five);
    print_type_of(&borrowed_five);
    println!("borrowed five {:?}", borrowed_five2);
    print_type_of(&borrowed_five2);
    println!();
}

fn main() {
    println!("=========================");
    refcell_example();
}
