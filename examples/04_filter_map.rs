pub fn filter_map_example() {
    let a = ["1", "two", "NaN", "four", "5"];
    let iter = a.iter().filter_map(|s| s.parse::<usize>().ok());
    for i in iter {
        println!("{:?}", i)
    }
}

fn main() {
    println!("=========================");
    filter_map_example();
}
