use crate::input::Input;
use crate::parse::{ParseIssue, Parser, Success};

pub struct OptParser<T, P> where P: Parser<Output=T> {
    parser: P,
}

impl<T, P> OptParser<T, P> where P: Parser<Output=T> {
    pub fn new(parser: P) -> Self {
        OptParser { parser }
    }
}

impl<T, P> Parser for OptParser<T, P> where P: Parser<Output=T> {
    type Output = Option<T>;

    fn parse<'a>(&self, input: &Input<'a>) -> Result<Success<'a, Self::Output>, ParseIssue> {
        let input = input.clone();
        match self.parser.parse(&input) {
            Ok(success) => Ok(Success {
                output: Some(success.output),
                remainder: success.remainder,
            }),
            Err(ParseIssue::Error(error)) => Err(ParseIssue::Error(error)),
            Err(ParseIssue::Failure(_)) => Ok(Success {
                output: None,
                remainder: input,
            }),
        }
    }
}