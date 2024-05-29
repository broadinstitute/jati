use crate::run::RunState;
use crate::trees::typed::tree::Tree;
use crate::trees::values::Value;

pub trait Executor<R: RunState, E: std::error::Error> {
    fn execute(&self, tree: Tree, state: &mut R) -> Result<Value, E>;
}

pub struct SimpleExecutor<R: RunState, E: std::error::Error> {}

impl<R: RunState, E: std::error::Error> SimpleExecutor<R, E> {
    pub fn new() -> Self { SimpleExecutor {} }
}

