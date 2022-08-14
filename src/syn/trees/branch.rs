use crate::syn::trees::Tree;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct Op {
    symbol: &'static str,
}

impl Op {
    pub fn new(symbol: &'static str) -> Op { Op { symbol } }
}

pub(crate) enum SyntaxPart {
    Op(&'static Op),
    Tree,
}

pub(crate) struct Syntax {
    parts: &'static [SyntaxPart],
}

impl Syntax {
    pub(crate) fn arity(&self) -> usize {
        self.parts.iter().filter(|part| { matches!(part, SyntaxPart::Tree) }).count()
    }
}

pub struct Branch {
    syntax: &'static Syntax,
    children: Vec<Tree>,
}