use std::marker::PhantomData;

use crate::runtime::Runtime;
use crate::trees::op::{NonIdOp, Op};
use crate::trees::props::Typed;
use crate::trees::tree::Tree;
use crate::trees::values::Value;

pub trait Executor<R: Runtime> {
    fn execute(&self, tree: &Tree<Typed>, runtime: &mut R, symbols: &mut R::S)
        -> Result<Value, R::E>;
}

pub struct SimpleExecutor<R: Runtime> {
    run_state_phantom: PhantomData<R>
}

impl<R: Runtime> SimpleExecutor<R> {
    pub fn new() -> Self {
        SimpleExecutor { run_state_phantom: Default::default() }
    }
}


impl<R: Runtime> Executor<R> for SimpleExecutor<R> {
    fn execute(&self, tree: &Tree<Typed>, runtime: &mut R, symbols: &mut R::S)
        -> Result<Value, R::E> {
        match tree {
            Tree::Var(var) => { runtime.get_var_value(&var.tag.key) }
            Tree::Literal(value) => { Ok(value.clone()) }
            Tree::Op(op) => {
                let mut values = Vec::with_capacity(op.kids.len());
                for kid in &op.kids {
                    values.push(self.execute(kid, runtime, symbols)?);
                    if runtime.stop_requested() { break; }
                }
                if runtime.stop_requested() {
                    Ok(Value::Never)
                } else {
                    match &op.op {
                        Op::NonId(non_id_op) => {
                            match non_id_op {
                                NonIdOp::BlockOpen => {
                                    values.last()
                                        .map_or(Ok(Value::Unit), |v| Ok(v.clone()))
                                }
                                NonIdOp::BlockClosed => {
                                    Ok(Value::Unit)
                                }
                            }
                        }
                        Op::Id(id_op) => {
                            runtime.apply_func(id_op.key(), &values, symbols)
                        }
                    }
                }
            }
        }
    }
}

impl<R: Runtime> Default for SimpleExecutor<R> {
    fn default() -> Self { SimpleExecutor::new() }
}