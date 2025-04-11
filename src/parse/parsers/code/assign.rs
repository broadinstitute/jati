use crate::parse::Parser;
use crate::trees::tree::Tree;
use crate::trees::props::Raw;
pub struct Assign<L, R> where L: Parser<Output=Tree<Raw>>, R: Parser<Output=Tree<Raw>> {
    left: L,
    right: R,
}

impl <L, R> Assign<L, R> where L: Parser<Output=Tree<Raw>>, R: Parser<Output=Tree<Raw>> {
    pub fn new(left: L, right: R) -> Self {
        Assign { left, right }
    }
}

impl Parser for Assign<L, R> where L: Parser<Output=Tree<Raw>>, R: Parser<Output=Tree<Raw>> {
    type Output = (Tree<Raw>, Tree<Raw>);
    fn parse<'a>(&self, input: &crate::input::Input<'a>) -> Result<crate::parse::Success<'a, Self::Output>, crate::parse::ParseIssue> {
        let Success { remainder: input1, output: output1 } = self.left.parse(input)?;
        let Success { remainder: input2, output: output2 } = self.right.parse(&input1)?;
        Ok(Success { output: (output1, output2), remainder: input2 })
    }
}

