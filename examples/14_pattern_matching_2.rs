#[derive(Debug)]
struct ExpensiveData {
    values: Vec<i32>,
}

#[derive(Debug)]
struct DataProcessor {
    source: Vec<i32>,
    cache: Option<ExpensiveData>,
}

impl DataProcessor {
    fn new(source: Vec<i32>) -> Self {
        Self {
            source,
            cache: None,
        }
    }

    fn get_or_compute(&mut self) -> &mut ExpensiveData {
        self.cache.get_or_insert({
            println!("Computing expensive data...");
            ExpensiveData {
                values: self.source.iter().map(|i| *i).collect(),
            }
        })
    }

    fn clear_cache(&mut self) {
        self.cache.take();
    }
}

fn main() {
    let mut processor = DataProcessor::new(vec![1, 2, 3, 4, 5]);
    let data = processor.get_or_compute();
    data.values.push(100);
    let data_again = processor.get_or_compute();
    println!("{:?}", data_again.values);
    processor.clear_cache();
    println!("{processor:?}");
}
