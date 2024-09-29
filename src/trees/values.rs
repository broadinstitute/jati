use std::fmt::{Display, Formatter};
use std::sync::Arc;
use crate::trees::props::Typed;
use crate::trees::tree::Tree;
use crate::trees::types::Type;

#[derive(Clone)]
pub enum Value {
    String(Arc<String>),
    Float(f64),
    Int(i64),
    Char(char),
    Bool(bool),
    Unit,
    Symbolic(Arc<Tree<Typed>>),
}

impl Value {
    pub fn tpe(&self) -> Type {
        match self {
            Value::String(_) => Type::String,
            Value::Float(_) => Type::Float,
            Value::Int(_) => Type::Int,
            Value::Char(_) => Type::Char,
            Value::Bool(_) => Type::Bool,
            Value::Unit => Type::Unit,
            Value::Symbolic(_) => Type::Symbolic
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(string) => { write!(f, "{string}") }
            Value::Float(number) => { write!(f, "{number}") }
            Value::Int(number) => { write!(f, "{number}") }
            Value::Char(char) => { write!(f, "{char}") }
            Value::Bool(bool) => { write!(f, "{bool}") }
            Value::Unit => { write!(f, "()") }
            Value::Symbolic(tree) => { write!(f, "{tree}") }
        }
    }
}