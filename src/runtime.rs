use crate::symbols::ops::{OpFn, OpKey};
use crate::symbols::symbol_table::SymbolTable;
use crate::symbols::var::VarKey;
use crate::trees::values::Value;

pub trait Runtime {
    type S: SymbolTable<Self>;
    type E: std::error::Error;
    fn request_stop(&mut self);
    fn stop_requested(&self) -> bool;
    fn set_var_value(&mut self, key: &VarKey, value: Value) -> Result<(), Self::E>;
    fn get_var_value(&self, key: &VarKey) -> Result<Value, Self::E>;
    fn get_op_func(&self, key: &OpKey) -> Result<OpFn<Self>, Self::E>;
}