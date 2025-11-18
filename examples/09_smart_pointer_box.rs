fn compare_box_array() {
    let a = [1; 1_000_000];
    let b = Box::new([2; 1_000_000]);
    println!("array size: {} bytes", std::mem::size_of_val(&a));
    // array size: 4000000 bytes
    println!("array size: {} bytes", std::mem::size_of_val(&b));
    // array size: 8 bytes
}

fn consuming_box(b: Box<i32>) {
    println!("{:?}", b);
}

fn borrow_box(b: &mut Box<i32>, val: i32) {
    **b = val;
}

fn main() {
    // consume & borrow
    let mut b: Box<i32> = Box::new(12);
    consuming_box(b.clone());
    borrow_box(&mut b, 13);
    println!("{:?}", &b);
    // size
    compare_box_array();
}
