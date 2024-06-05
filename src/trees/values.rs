use std::fmt::{Display, Formatter};
use std::sync::Arc;
use crate::trees::typed::tree::Tree;
use crate::trees::types::Type;

#[derive(Clone)]
pub enum Value {
    String(Arc<String>),
    Float(f64),
    Int(i64),
    Char(char),
    Bool(bool),
    Unit,
    Never,
    Symbolic(Arc<Tree>),
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
            Value::Never => Type::Never,
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
            Value::Never => { write!(f, "!") }
            Value::Symbolic(tree) => { write!(f, "{tree}") }
        }
    }
}