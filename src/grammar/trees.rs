use crate::grammar::trees::composite::Composite;
use crate::grammar::trees::lit::Literal;
use crate::grammar::trees::var::Var;

pub mod composite;
pub mod var;
pub mod lit;

pub enum Tree {
    Composite(Box<dyn Composite>),
    Var(Var),
    Lit(Literal)
}
