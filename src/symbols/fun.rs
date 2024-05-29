use std::sync::Arc;
use uuid::Uuid;
use crate::error::Error;
use crate::run::RunState;
use crate::symbols::symbol_table::SymbolTable;
use crate::trees::symbols::ArgsFailure;
use crate::trees::types::Type;
use crate::trees::values::Value;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FunKey {
    uuid: Uuid,
}

impl FunKey {
    pub fn new(uuid: Uuid) -> FunKey { FunKey { uuid } }
    pub fn next() -> FunKey {
        let uuid = Uuid::new_v4();
        FunKey { uuid }
    }
}

#[derive(Clone)]
pub enum FunSig {
    Fixed { tpe: Type, arg_types: Vec<Type> }
}

impl FunSig {
    pub(crate) fn tpe(&self) -> Type {
        match self {
            FunSig::Fixed { tpe, .. } => *tpe
        }
    }

    pub(crate) fn check_arg_types(&self, arg_types: &[Type]) -> Result<(), ArgsFailure> {
        match self {
            FunSig::Fixed { arg_types: expected, .. } => {
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
pub struct FunTag {
    pub key: FunKey,
    pub sig: Arc<FunSig>,
}

pub struct PreDefFun<'a, R: RunState, S: SymbolTable> {
    pub name: &'a str,
    pub uuid: Uuid,
    pub sig: FunSig,
    pub run: fn(args: &[Value], &mut R, &mut S) -> Result<Value, Error>
}



