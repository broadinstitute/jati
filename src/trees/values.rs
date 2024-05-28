use crate::trees::typed::tree::Tree;
use crate::trees::types::Type;

pub enum Value {
    String(String),
    Float(f64),
    Int(i64),
    Char(char),
    Bool(bool),
    Unit,
    Never,
    Symbolic(Tree)
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