use crate::trees::types::Type;

#[derive(Clone)]
pub enum Literal {
    String(String),
    Float(f64),
    Int(i64),
    Char(char),
    Bool(bool),
    Unit,
}

impl Literal {
    pub fn tpe(&self) -> Type {
        match self {
            Literal::String(_) => { Type::String }
            Literal::Float(_) => { Type::Float }
            Literal::Int(_) => { Type::Int }
            Literal::Char(_) => { Type::Char }
            Literal::Bool(_) => { Type::Bool }
            Literal::Unit => { Type::Unit }
        }
    }
}
