use crate::engine::fun::Fun;
use crate::engine::var::Var;
use crate::trees::types::Type;

pub trait Symbols<V: Var, F: Fun> {
    fn get_var(&mut self, name: &str) -> Result<V, SymbolError>;
    fn get_fun(&mut self, name: &str, args: Vec<Type>) -> Result<F, SymbolError>;
}

pub enum SymbolError {
    NoSuchVar(String),
    NoSuchFun(String),
    Args(ArgsError),
}

impl SymbolError {
    pub fn no_such_var(name: &str) -> SymbolError { SymbolError::NoSuchVar(String::from(name)) }
    pub fn no_such_fun(name: &str) -> SymbolError { SymbolError::NoSuchFun(String::from(name)) }
    pub fn args_issue(name: &str, args_failure: ArgsFailure) -> SymbolError {
        let fun_name = String::from(name);
        let args_error = ArgsError { fun_name, args_failure };
        SymbolError::Args(args_error)
    }
    pub fn wrong_number_of_args(name: &str, actual: usize, expected: usize) -> SymbolError {
        let fun_name = String::from(name);
        let args_failure = ArgsFailure::WrongNumber { actual, expected };
        SymbolError::Args(ArgsError { fun_name, args_failure })
    }
    pub fn message(&self) -> String {
        match self {
            SymbolError::NoSuchVar(name) => { format!("Unknown variable {}.", name) }
            SymbolError::NoSuchFun(name) => { format!("Unknown function {}.", name) }
            SymbolError::Args(args_error) => {
                let ArgsError { fun_name, args_failure } = args_error;
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

pub struct ArgsError {
    fun_name: String,
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
