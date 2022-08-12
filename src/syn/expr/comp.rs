use crate::syn::expr::Expr;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Op {
    symbol: &'static str,
}

impl Op {
    pub fn new(symbol: &'static str) -> Op { Op { symbol } }
}

pub enum ExprPart {
    Op(Op),
    Expr(Expr)
}

pub struct CompExpr {
    parts: Vec<ExprPart>
}