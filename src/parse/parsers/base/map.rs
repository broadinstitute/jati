use crate::input::Input;
use crate::parse::{ParseIssue, Parser, Success};

pub struct MapParser<T, T2, P, F> where P: Parser<Output=T>, F: Fn(T) -> T2 {
    parser: P,
    f: F,
}

impl<T, T2, P, F> MapParser<T, T2, P, F> where P: Parser<Output=T>, F: Fn(T) -> T2 {
    pub fn new(parser: P, f: F) -> Self {
        MapParser { parser, f }
    }
}

impl <T, T2, P, F> Parser for MapParser<T, T2, P, F> where P: Parser<Output=T>, F: Fn(T) -> T2 {
    type Output = T2;

    fn parse<'a>(&self, input: &Input<'a>) -> Result<Success<'a, Self::Output>, ParseIssue> {
        let Success { output, remainder } = self.parser.parse(input)?;
        Ok(Success {
            output: (self.f)(output),
            remainder,
        })
    }
}