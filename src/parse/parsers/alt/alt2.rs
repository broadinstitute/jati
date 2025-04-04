use crate::input::{CharTap, Input};
use crate::parse::{ParseIssue, Parser, Success};

pub struct Alt2Parser<T, P1, P2>
where
    P1: Parser<Output = T>,
    P2: Parser<Output = T>,
{
    parser1: P1,
    parser2: P2,
}

impl<T, P1, P2> Alt2Parser<T, P1, P2>
where
    P1: Parser<Output = T>,
    P2: Parser<Output = T>,
{
    pub fn new(parser1: P1, parser2: P2) -> Self {
        Alt2Parser { parser1, parser2 }
    }
}

impl<T, P1, P2> Parser for Alt2Parser<T, P1, P2>
where
    P1: Parser<Output = T>,
    P2: Parser<Output = T>,
{
    type Output = T;

    fn parse<C: CharTap>(&self, input: &Input<C>) -> Result<Success<C, Self::Output>, ParseIssue> {
        let input = input.clone();
        match self.parser1.parse(&input) {
            Ok(success) => Ok(success),
            Err(ParseIssue::Error(error)) => { Err(ParseIssue::Error(error)) }
            Err(ParseIssue::Failure(failure1)) => {
                match self.parser2.parse(&input) {
                    Ok(success) => Ok(success),
                    Err(ParseIssue::Error(error)) => { Err(ParseIssue::Error(error)) }
                    Err(ParseIssue::Failure(failure2)) => {
                        let failure = failure1.alt_combine(failure2);
                        Err(ParseIssue::Failure(failure))
                    }
                }
            }
        }
    }
}