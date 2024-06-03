use crate::symbols::var::VarKey;
use crate::trees::values::Value;

pub trait RunState<E: std::error::Error> {
    fn request_stop(&mut self);
    fn stop_requested(&self) -> bool;
    fn set_var_value(&mut self, key: VarKey, value: Value) -> Result<(), E>;
    fn get_var_value(&self, key: VarKey) -> Result<Value, E>;
}