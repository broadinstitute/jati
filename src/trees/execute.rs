use std::marker::PhantomData;

use crate::runtime::Runtime;
use crate::trees::typed::op::{NonIdOp, Op};
use crate::trees::typed::tree::Tree;
use crate::trees::values::Value;

pub trait Executor<R: Runtime> {
    fn execute(&self, tree: &Tree, runtime: &mut R, symbols: &mut R::S) -> Result<Value, R::E>;
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
    fn execute(&self, tree: &Tree, runtime: &mut R, symbols: &mut R::S) -> Result<Value, R::E> {
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
                                NonIdOp::Block => {
                                    values.last()
                                        .map_or(Ok(Value::Unit), |v| Ok(v.clone()))
                                }
                            }
                        }
                        Op::Id(id_op) => {
                            let op_fn = runtime.get_op_func(id_op.key())?;
                            (op_fn.func)(&values, runtime, symbols)
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