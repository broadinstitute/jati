use crate::engine::fun::Fun;
use crate::engine::var::Var;
use crate::error::Error;
use crate::trees::types::Type;

pub trait Symbols<V: Var, F: Fun> {
    fn get_var(&mut self, name: &str) -> Result<V, Error>;
    fn get_fun(&mut self, name: &str, args: Vec<Type>) -> Result<F, Error>;
}