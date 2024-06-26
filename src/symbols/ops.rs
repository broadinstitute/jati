use std::sync::Arc;

use uuid::Uuid;

use crate::runtime::Runtime;
use crate::trees::symbols::ArgsFailure;
use crate::trees::types::Type;
use crate::trees::values::Value;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct OpKey {
    uuid: Uuid,
}

impl OpKey {
    pub fn new(uuid: Uuid) -> OpKey { OpKey { uuid } }
    pub fn create_random() -> OpKey {
        let uuid = Uuid::new_v4();
        OpKey { uuid }
    }
}

#[derive(Clone)]
pub enum OpSig {
    Fixed { tpe: Type, arg_types: Vec<Type> }
}

impl OpSig {
    pub(crate) fn tpe(&self) -> Type {
        match self {
            OpSig::Fixed { tpe, .. } => *tpe
        }
    }

    pub(crate) fn check_arg_types(&self, arg_types: &[Type]) -> Result<(), ArgsFailure> {
        match self {
            OpSig::Fixed { arg_types: expected, .. } => {
                if arg_types.len() != expected.len() {
                    Err(ArgsFailure::new_wrong_number(arg_types.len(), expected.len()))?
                }
                for (pos, (&actual, &expected))
                in arg_types.iter().zip(expected.iter()).enumerate() {
                    if actual != expected {
                        Err(ArgsFailure::new_wrong_type(pos, actual, expected))?
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone)]
pub struct OpTag {
    pub key: OpKey,
    pub sig: Arc<OpSig>,
}

pub type Func<R> =
fn(args: &[Value], &mut R, &mut <R as Runtime>::S) -> Result<Value, <R as Runtime>::E>;

pub struct OpFn<R: Runtime + ?Sized> {
    pub func: Func<R>,
}

impl<R: Runtime> OpFn<R> {
    pub const fn new(func: Func<R>) -> OpFn<R> {
        OpFn { func }
    }
}

impl<R: Runtime> From<fn(args: &[Value], &mut R, &mut <R as Runtime>::S)
    -> Result<Value, <R as Runtime>::E>> for OpFn<R> {
    fn from(func: fn(args: &[Value], &mut R, &mut <R as Runtime>::S)
        -> Result<Value, <R as Runtime>::E>) -> OpFn<R> {
        OpFn { func }
    }
}

pub struct OpPreDef<'a, R: Runtime + ?Sized> {
    pub name: &'a str,
    pub sig: OpSig,
    pub func: OpFn<R>,
}



