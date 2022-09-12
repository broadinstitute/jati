use crate::grammar::trees::composite::Composite;
use crate::grammar::trees::lit::Literal;
use crate::grammar::trees::id::Id;

pub mod composite;
pub mod id;
pub mod lit;
pub mod call;

pub enum Tree {
    Composite(Box<dyn Composite>),
    Var(Id),
    Lit(Literal)
}
