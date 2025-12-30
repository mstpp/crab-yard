/// A processor that transforms items while holding a reference to shared context
trait Processor<'ctx> {
    type Item;
    type Output;

    fn process(&self, item: Self::Item, context: &'ctx str) -> Self::Output;
}

/// Filters log lines by keyword, borrowing the keyword string
struct LogFilter<'a> {
    keyword: &'a str,
}

impl<'a, 'ctx> Processor<'ctx> for LogFilter<'a> {
    type Item = &'ctx str;
    type Output = Option<&'ctx str>;

    fn process(&self, item: Self::Item, _context: &'ctx str) -> Self::Output {
        if item.contains(self.keyword) {
            Some(item)
        } else {
            None
        }
    }
}

/// Chains two processors together
struct Chain<A, B> {
    first: A,
    second: B,
}

// Task: Implement Processor for Chain where A and B are both Processors
// This is the hard part - getting the lifetime bounds right
impl<'ctx, A, B> Processor<'ctx> for Chain<A, B>
where
    A: Processor<'ctx, Item = &'ctx str, Output = Option<&'ctx str>>,
    B: Processor<'ctx, Item = &'ctx str, Output = Option<&'ctx str>>,
{
    type Item = &'ctx str;
    type Output = Option<&'ctx str>;

    fn process(&self, item: Self::Item, context: &'ctx str) -> Self::Output {
        // if let Some(_) = self.first.process(item, context) {
        //     if let Some(_) = self.second.process(item, context) {
        //         Some(item)
        //     } else {
        //         None
        //     }
        // } else {
        //     None
        // }
        self.first
            .process(item, context)
            .and_then(|result| self.second.process(result, context))
    }
}

fn main() {
    let level = String::from("ERROR");
    let filter = LogFilter { keyword: &level };

    let logs = vec![
        "[INFO] Started",
        "[ERROR] Failed to connect",
        "[ERROR] Timeout",
    ];

    let context = "production";

    for log in logs {
        if let Some(filtered) = filter.process(log, context) {
            println!("Matched: {}", filtered);
        }
    }
    // processor chain
    let log_level = "INFO";
    let kw = "Started";

    let info_filter = LogFilter { keyword: log_level };
    let started_filter = LogFilter { keyword: kw };

    let chain = Chain {
        first: info_filter,
        second: started_filter,
    };
    let logs = vec![
        "[INFO] Started",
        "[WARN] Config not set",
        "[ERROR] Failed to connect",
        "[ERROR] Timeout",
        "[INFO] Re-Started",
    ];
    for log in logs {
        if let Some(filtered) = chain.process(log, context) {
            println!("Chain-matched: {}", filtered);
        }
    }
}
