use std::marker::PhantomData;
use crate::run::RunState;
use crate::trees::typed::tree::Tree;
use crate::trees::values::Value;

pub trait Executor<R: RunState, E: std::error::Error> {
    fn execute(&self, tree: Tree, state: &mut R) -> Result<Value, E>;
}

pub struct SimpleExecutor<R: RunState, E: std::error::Error> {
    run_state_phantom: PhantomData<R>,
    error_phantom: PhantomData<E>,
}

impl<R: RunState, E: std::error::Error> SimpleExecutor<R, E> {
    pub fn new() -> Self { SimpleExecutor { run_state_phantom: Default::default(), error_phantom: Default::default() } }
}


impl<R: RunState, E: std::error::Error> Executor<R, E> for SimpleExecutor<R, E> {
    fn execute(&self, tree: Tree, state: &mut R) -> Result<Value, E> {
        match tree {
            Tree::Var(_) => { todo!() }
            Tree::Literal(_) => { todo!() }
            Tree::Operation(_) => { todo!() }
        }
    }
}

impl<R: RunState, E: std::error::Error> Default for SimpleExecutor<R, E> {
    fn default() -> Self { SimpleExecutor::new() }
}