use std::fmt::{Debug, Display, Formatter};
use crate::symbols::id::Id;
use crate::trees::types::Type;

pub enum SymbolError {
    NoSuchVar(Id),
    NoSuchFun(Id),
    Args(ArgsError),
}

impl SymbolError {
    pub fn no_such_var(id: &Id) -> SymbolError { SymbolError::NoSuchVar(id.clone()) }
    pub fn no_such_fun(id: &Id) -> SymbolError { SymbolError::NoSuchFun(id.clone()) }
    pub fn args_issue(id: &Id, args_failure: ArgsFailure) -> SymbolError {
        let fun_name = id.clone();
        let args_error = ArgsError { fun_id: fun_name, args_failure };
        SymbolError::Args(args_error)
    }
    pub fn message(&self) -> String {
        match self {
            SymbolError::NoSuchVar(name) => { format!("Unknown variable {}.", name) }
            SymbolError::NoSuchFun(name) => { format!("Unknown function {}.", name) }
            SymbolError::Args(args_error) => {
                let ArgsError { fun_id: fun_name, args_failure } = args_error;
                match args_failure {
                    ArgsFailure::WrongNumber { actual, expected } => {
                        format!("Function {} needs {} args, but found {}.", fun_name, expected,
                                actual)
                    }
                    ArgsFailure::WrongType { pos, actual, expected } => {
                        format!("Function {}'s arg {} should be of type {}, but found {}.",
                                fun_name, pos, expected, actual)
                    }
                }
            }
        }
    }
}

impl Display for SymbolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

pub struct ArgsError {
    fun_id: Id,
    args_failure: ArgsFailure,
}

pub enum ArgsFailure {
    WrongNumber { actual: usize, expected: usize },
    WrongType { pos: usize, actual: Type, expected: Type },
}

impl ArgsError {
    pub fn new(fun_id: Id, args_failure: ArgsFailure) -> ArgsError {
        ArgsError { fun_id, args_failure }
    }
}

impl ArgsFailure {
    pub fn new_wrong_number(actual: usize, expected: usize) -> ArgsFailure {
        ArgsFailure::WrongNumber { actual, expected }
    }
    pub fn new_wrong_type(pos: usize, actual: Type, expected: Type) -> ArgsFailure {
        ArgsFailure::WrongType { pos, actual, expected }
    }
}

impl Debug for SymbolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { Display::fmt(self, f) }
}

impl std::error::Error for SymbolError {}

impl From<ArgsError> for SymbolError {
    fn from(args_error: ArgsError) -> Self { SymbolError::Args(args_error) }
}
