trait Parser<'a> {
    type Output;

    // helper: return tuple, first parsed value and a remainder for chaining
    fn parse_find(&self, input: &'a str) -> Option<(Self::Output, &'a str)>;

    // default impl (optional to implement)
    fn parse(&self, input: &'a str) -> Option<Self::Output> {
        self.parse_find(input).map(|(val, _remainder)| val)
    }
}

// custom struct, int parser and implementing Parser trait
struct IntParser;

impl<'a> Parser<'a> for IntParser {
    type Output = u32;

    fn parse_find(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
        let parsed: Self::Output = input
            .split(|c: char| !c.is_ascii_digit())
            .find_map(|s| s.parse().ok())?;
        let remainder = input.split_once(parsed.to_string().as_str())?.1;
        Some((parsed, remainder))
    }
}

// Parse a pair, must have Some for both Parsers
struct PairParser<T1, T2>(T1, T2);

impl<'a, T1, T2> Parser<'a> for PairParser<T1, T2>
where
    T1: Parser<'a>,
    T2: Parser<'a>,
{
    type Output = (T1::Output, T2::Output);

    fn parse_find(&self, input: &'a str) -> Option<(Self::Output, &'a str)> {
        self.0.parse_find(input).and_then(|res| {
            let (t2, remain) = self.1.parse_find(res.1)?;
            Some(((res.0, t2), remain))
        })
    }
}

fn main() {
    assert_eq!(IntParser.parse("abc123.456---"), Some(123));
    let p = PairParser(IntParser, IntParser);
    assert_eq!(p.parse("42,17"), Some((42, 17)));
}
