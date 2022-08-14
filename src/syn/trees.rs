use crate::syn::trees::branch::Branch;
use crate::syn::trees::lit::Literal;
use crate::syn::trees::var::Var;

pub mod branch;
pub mod var;
pub mod lit;

pub enum Tree {
    Branch(Branch),
    Var(Var),
    Lit(Literal)
}
