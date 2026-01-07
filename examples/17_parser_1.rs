trait Parser {
    type Output;

    fn parse(&self, input: &str) -> Option<Self::Output>;
    // TODO add parser that returns remainder
    // TODO add parser that mutate in plance the input
}

struct IntParser;

impl Parser for IntParser {
    type Output = u32;

    fn parse(&self, input: &str) -> Option<Self::Output> {
        input
            .split(|c: char| !c.is_ascii_digit())
            .find_map(|s| s.parse().ok())
    }
}

struct PairParser<T, OutType>(T, T)
where
    T: Parser<Output = OutType>;

impl<T, OutType> Parser for PairParser<T, OutType>
where
    T: Parser<Output = OutType>,
{
    type Output = (OutType, OutType);

    fn parse(&self, input: &str) -> Option<Self::Output> {
        Some((self.0.parse(input)?, self.1.parse(input)?))
    }
}
fn main() {
    let p = PairParser(IntParser, IntParser);
    assert_eq!(p.parse("42,17"), Some((42, 17)));
}
