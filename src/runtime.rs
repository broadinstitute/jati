use crate::symbols::symbol_table::SymbolTable;

pub trait Runtime {
    type S: SymbolTable<Self>;
    type E: std::error::Error;
    fn request_stop(&mut self);
    fn stop_requested(&self) -> bool;
}