use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Type {
    String,
    Float,
    Int,
    Char,
    Bool,
    Unit,
    Never,
    Symbolic
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::String => write!(f, "String"),
            Type::Float => write!(f, "Float"),
            Type::Int => write!(f, "Int"),
            Type::Char => write!(f, "Char"),
            Type::Bool => write!(f, "Bool"),
            Type::Unit => write!(f, "Unit"),
            Type::Never => write!(f, "Never"),
            Type::Symbolic => write!(f, "Symbolic")
        }
    }
}


