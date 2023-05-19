use crate::error::Error;
use crate::symbols::fun::FunTag;
use crate::symbols::var::VarTag;
use crate::symbols::id::Id;
use crate::trees::types::Type;

pub trait SymbolTable {
    fn get_var(&mut self, id: &Id) -> Result<Option<VarTag>, Error>;
    fn get_fun(&mut self, id: &Id, args: &[Type]) -> Result<Option<FunTag>, Error>;
}