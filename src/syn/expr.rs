use crate::syn::expr::comp::CompExpr;
use crate::syn::expr::lit::Literal;
use crate::syn::expr::var::Var;

pub mod comp;
pub mod var;
pub mod lit;

pub enum Expr {
    Comp(CompExpr),
    Var(Var),
    Lit(Literal)
}
