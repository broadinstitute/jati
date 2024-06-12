use crate::error::Error;
use crate::symbols::symbol_table::SymbolTable;

pub trait Runtime {
    type S: SymbolTable<Self>;
    type E: std::error::Error + From<Error>;
    fn request_stop(&mut self);
    fn stop_requested(&self) -> bool;
}