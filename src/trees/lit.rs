use crate::trees::types::Type;

pub enum Literal {
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
}

impl Literal {
    pub fn tpe(&self) -> Type {
        match self {
            Literal::String(_) => { Type::String }
            Literal::Float(_) => { Type::Float }
            Literal::Int(_) => { Type::Int }
            Literal::Bool(_) => { Type::Bool }
        }
    }
}
