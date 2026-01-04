use std::boxed::Box;

type ParseResult<'a, T> = Option<(T, &'a str)>;
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

fn seq<'a, A, B>(parser_a: Parser<'a, A>, parser_b: Parser<'a, B>) -> Parser<'a, (A, B)>
where
    A: 'a,
    B: 'a,
{
    Box::new(move |input| {
        if let Some((a, remain)) = parser_a(input) {
            if let Some((b, remain)) = parser_b(remain) {
                return Some(((a, b), remain));
            }
            return None;
        }
        None
    })
}

fn main() {
    // Parser that takes until first space
    let parse_word: Parser<&str> = Box::new(|input| {
        let end = input.find(' ').unwrap_or(input.len());
        let (word, rest) = input.split_at(end);
        Some((word, rest.trim_start()))
    });

    // Parser that takes until colon
    let parse_time_part: Parser<&str> = Box::new(|input| {
        let (part, rest) = input.split_once(':')?;
        Some((part, rest))
    });

    // Combine them!
    let combined = seq(parse_time_part, parse_word);
    let result = combined("10:20 Info Good morning!");

    println!("{:?}", result);
}
