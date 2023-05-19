use std::fmt::{Display, Formatter};
use crate::symbols::id::Id;

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
    WrongNumber { actual: usize, expected: usize }
}

impl ArgsFailure {
    pub fn new_wrong_number(actual: usize, expected: usize) -> ArgsFailure {
        ArgsFailure::WrongNumber { actual, expected}
    }
}
