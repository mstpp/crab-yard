trait Processor {
    type Error; // associated type, don't forget to implement

    fn process(&self, input: &str) -> Result<String, Self::Error>;
    fn name(&self) -> &'static str;
}

#[derive(Debug)]
struct UpperCase;

#[derive(Debug)]
enum ProcessorError {
    InvalidConfig,
}

impl Processor for UpperCase {
    type Error = ProcessorError;

    fn name(&self) -> &'static str {
        "UpperCase"
    }

    fn process(&self, input: &str) -> Result<String, Self::Error> {
        Ok(input.to_ascii_uppercase())
    }
}

#[derive(Debug)]
struct Censor {
    censored: Vec<String>,
}

impl Censor {
    fn new<I, S>(words: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Censor {
            censored: words.into_iter().map(Into::into).collect(),
        }
    }
}

impl Processor for Censor {
    type Error = ProcessorError;

    fn name(&self) -> &'static str {
        "Censor"
    }

    fn process(&self, input: &str) -> Result<String, Self::Error> {
        let mut res = String::from(input);
        for bad_word in self.censored.iter() {
            res = res.replace(bad_word, "*".repeat(bad_word.len()).as_str());
        }
        Ok(res)
    }
}

#[derive(Debug)]
struct Truncate {
    size: usize,
}

impl Truncate {
    fn new(size: usize) -> Self {
        Truncate { size }
    }
}

impl Processor for Truncate {
    type Error = ProcessorError;

    fn process(&self, input: &str) -> Result<String, Self::Error> {
        if self.size == 0 {
            return Err(ProcessorError::InvalidConfig);
        }
        Ok(input.chars().take(self.size).collect())
    }

    fn name(&self) -> &'static str {
        "Truncate"
    }
}

#[derive(Debug)]
struct Reverse;

impl Processor for Reverse {
    type Error = ProcessorError;

    fn process(&self, input: &str) -> Result<String, Self::Error> {
        Ok(input.chars().rev().collect())
    }

    fn name(&self) -> &'static str {
        "Reverse"
    }
}

// Processor Pipeline
type ProcBox = Box<dyn Processor<Error = ProcessorError>>;
type ProcVec = Vec<ProcBox>;

struct Pipeline {
    procs: ProcVec,
}

impl Pipeline {
    fn new() -> Self {
        Pipeline { procs: Vec::new() }
    }

    fn add(&mut self, proc: ProcBox) {
        self.procs.push(proc);
    }

    fn run(&self, input: &str) -> Result<String, ProcessorError> {
        self.procs
            .iter()
            .try_fold(input.to_string(), |current, proc| {
                println!("Input:{}\n{}-ing...", &current, proc.name());
                proc.process(&current)
            })
    }
}

fn main() -> Result<(), ProcessorError> {
    let mut pipeline = Pipeline::new();
    pipeline.add(Box::new(UpperCase));
    pipeline.add(Box::new(Censor::new(vec!["BAD", "EVIL"])));
    pipeline.add(Box::new(Truncate::new(35)));
    let result = pipeline.run("This is a bad and evil example text======")?;
    // result: "THIS IS A *** AND *** EXAMPLE TEXT"
    println!("{result}");
    println!("Reversed: {:?}", Reverse.process(&result));
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_empty_uppercase() {
        let proc = UpperCase;
        let res = proc.process("");
        assert!(res.is_ok())
    }
}
