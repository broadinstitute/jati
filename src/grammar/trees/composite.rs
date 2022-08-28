use crate::grammar::trees::Tree;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Op {
    symbol: &'static str,
}

impl Op {
    pub fn new(symbol: &'static str) -> Op { Op { symbol } }
}

pub trait Composite {
    fn children(&self) -> Vec<Tree>;
}