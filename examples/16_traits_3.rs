use std::fmt::Debug;

trait Loggable {
    fn log(&self); // prints "[LOG] {debug_representation}"
    fn log_with_prefix(&self, prefix: &str);
    fn to_log_string(&self) -> String;
}

// BLANKET implementation so any Debug type gets Loggable for free
impl<T: Debug> Loggable for T {
    fn log(&self) {
        println!("[LOG] {:?}", self);
    }

    fn log_with_prefix(&self, prefix: &str) {
        println!("[{}] {:?}", prefix, self);
    }

    fn to_log_string(&self) -> String {
        format!("[LOG] {:?}", self)
    }
}
fn main() {
    vec![1, 2, 3].log();
    "hello".log_with_prefix("DEBUG");
    println!("{}", 111.to_log_string());
}
