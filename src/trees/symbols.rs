use crate::engine::fun::Fun;
use crate::engine::var::Var;
use crate::error::Error;
use crate::trees::types::Type;

pub trait Symbols<V: Var, F: Fun> {
    fn get_var(&mut self, name: &str) -> Result<V, Error>;
    fn get_fun(&mut self, name: &str, args: Vec<Type>) -> Result<F, Error>;
}

pub mod errors {
    use crate::error::Error;

    pub fn no_such_var(name: &str) -> Error {
        Error::new_symbols_error(format!("Unknown variable {}.", name))
    }
    pub fn no_such_fun(name: &str) -> Error {
        Error::new_symbols_error(format!("Unknown function {}.", name))
    }
    pub fn wrong_number_of_args(name: &str, n_actual: usize, n_expected: usize) -> Error {
        Error::new_symbols_error(format!("Function {} needs {} args, but found {}.",
                                         name, n_expected, n_actual))
    }
}