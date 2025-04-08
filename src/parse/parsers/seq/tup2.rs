use crate::input::Input;
use crate::parse::{ParseIssue, Parser, Success};

pub struct Tup2Parser<T1, T2, P1, P2>
where
    P1: Parser<Output = T1>,
    P2: Parser<Output = T2>,
{
    parser1: P1,
    parser2: P2,
}

impl<T1, T2, P1, P2> Tup2Parser<T1, T2, P1, P2>
where
    P1: Parser<Output = T1>,
    P2: Parser<Output = T2>,
{
    pub fn new(parser1: P1, parser2: P2) -> Self {
        Tup2Parser { parser1, parser2 }
    }
}

impl<T1, T2, P1, P2> Parser for Tup2Parser<T1, T2, P1, P2>
where
    P1: Parser<Output = T1>,
    P2: Parser<Output = T2>,
{
    type Output = (T1, T2);

    fn parse<'a>(&self, input: &Input<'a>) -> Result<Success<'a, Self::Output>, ParseIssue> {
        let Success {
            remainder: input1,
            output: output1,
        } = self.parser1.parse(input)?;
        let Success {
            remainder: input2,
            output: output2,
        } = self.parser2.parse(&input1)?;
        Ok(Success {
            output: (output1, output2),
            remainder: input2,
        })
    }
}
